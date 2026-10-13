//! Module items: imports and exports (ECMA-262 §16.2).

use super::{PResult, Parser};
use crate::ast::*;
use crate::lexer::{P, Tok};

impl Parser<'_> {
    pub fn parse_module_items(&mut self) -> PResult<Vec<ModuleItem>> {
        let mut items = Vec::new();
        while self.tok.kind != Tok::Eof {
            if self.is_word("import") {
                let next = self.peek()?;
                if !next.is(P::LParen) && !next.is(P::Dot) {
                    items.push(ModuleItem::Import(self.parse_import()?));
                    continue;
                }
            }
            if self.is_word("export") {
                items.push(ModuleItem::Export(self.parse_export()?));
                continue;
            }
            items.push(ModuleItem::Stmt(self.parse_statement_list_item()?));
        }
        Ok(items)
    }

    fn parse_module_specifier(&mut self) -> PResult<Utf16> {
        if self.tok.kind != Tok::String {
            return self.unexpected();
        }
        let t = self.next()?;
        Ok(t.str_val)
    }

    /// `with { key: "value", ... }`
    fn parse_import_attributes(&mut self) -> PResult<Vec<ImportAttribute>> {
        if !self.is_word("with") {
            return Ok(Vec::new());
        }
        self.next()?;
        self.expect(P::LBrace)?;
        let mut attrs: Vec<ImportAttribute> = Vec::new();
        while !self.is(P::RBrace) {
            let key = match self.tok.kind {
                Tok::Ident => self.next()?.ident.encode_utf16().collect(),
                Tok::String => self.next()?.str_val,
                _ => return self.unexpected(),
            };
            self.expect(P::Colon)?;
            if self.tok.kind != Tok::String {
                return self.unexpected();
            }
            let value = self.next()?.str_val;
            if attrs.iter().any(|a| a.key == key) {
                return self.err("duplicate import attribute", self.prev_end);
            }
            attrs.push(ImportAttribute { key, value });
            if !self.is(P::RBrace) {
                self.expect(P::Comma)?;
            }
        }
        self.next()?;
        Ok(attrs)
    }

    fn parse_module_export_name(&mut self) -> PResult<ModuleExportName> {
        match self.tok.kind {
            Tok::Ident => Ok(ModuleExportName::Ident(self.next()?.ident)),
            Tok::String => {
                let t = self.next()?;
                if String::from_utf16(&t.str_val).is_err() {
                    return self.err("module export names must be well-formed Unicode", t.start);
                }
                Ok(ModuleExportName::String(t.str_val))
            }
            _ => self.unexpected(),
        }
    }

    fn parse_import(&mut self) -> PResult<ImportDecl> {
        let start = self.start();
        self.next()?; // import
        let mut decl = ImportDecl {
            default: None,
            namespace: None,
            named: Vec::new(),
            source: Vec::new(),
            attributes: Vec::new(),
            span: Span::default(),
        };
        if self.tok.kind == Tok::String {
            decl.source = self.parse_module_specifier()?;
            decl.attributes = self.parse_import_attributes()?;
            self.semicolon()?;
            decl.span = self.span_from(start);
            return Ok(decl);
        }
        if self.tok.kind == Tok::Ident && !self.is_word("from")
            || self.is_word("from") && self.peek()?.is_from_or_comma()
        {
            decl.default = Some(self.binding_identifier()?);
            if !self.eat(P::Comma)? {
                return self.finish_import(start, decl);
            }
        }
        if self.eat(P::Star)? {
            self.expect_word("as")?;
            decl.namespace = Some(self.binding_identifier()?);
        } else if self.eat(P::LBrace)? {
            while !self.is(P::RBrace) {
                let imported = self.parse_module_export_name()?;
                let local = if self.eat_word("as")? {
                    self.binding_identifier()?
                } else {
                    match &imported {
                        ModuleExportName::Ident(name) => {
                            let span = Span::new(self.tok.start, self.tok.start);
                            self.check_identifier(name, self.prev_end, true)?;
                            Ident {
                                name: name.clone(),
                                span,
                            }
                        }
                        ModuleExportName::String(_) => return self.unexpected(),
                    }
                };
                decl.named.push(ImportSpecifier { imported, local });
                if !self.is(P::RBrace) {
                    self.expect(P::Comma)?;
                }
            }
            self.next()?;
        } else {
            return self.unexpected();
        }
        self.finish_import(start, decl)
    }

    fn finish_import(&mut self, start: usize, mut decl: ImportDecl) -> PResult<ImportDecl> {
        self.expect_word("from")?;
        decl.source = self.parse_module_specifier()?;
        decl.attributes = self.parse_import_attributes()?;
        self.semicolon()?;
        decl.span = self.span_from(start);
        Ok(decl)
    }

    fn parse_export(&mut self) -> PResult<ExportDecl> {
        let start = self.start();
        self.next()?; // export
        if self.eat(P::Star)? {
            let alias = if self.eat_word("as")? {
                Some(self.parse_module_export_name()?)
            } else {
                None
            };
            self.expect_word("from")?;
            let source = self.parse_module_specifier()?;
            let attributes = self.parse_import_attributes()?;
            self.semicolon()?;
            return Ok(ExportDecl::All {
                alias,
                source,
                attributes,
                span: self.span_from(start),
            });
        }
        if self.eat(P::LBrace)? {
            let mut specifiers = Vec::new();
            let mut first_reserved: Option<usize> = None;
            while !self.is(P::RBrace) {
                let pos = self.tok.start;
                let is_string = self.tok.kind == Tok::String;
                let local = self.parse_module_export_name()?;
                if let ModuleExportName::Ident(name) = &local
                    && first_reserved.is_none()
                    && (crate::RESERVED_WORDS.contains(&name.as_str())
                        || crate::STRICT_RESERVED_WORDS.contains(&name.as_str()))
                {
                    first_reserved = Some(pos);
                }
                if is_string && first_reserved.is_none() {
                    first_reserved = Some(pos);
                }
                let exported = if self.eat_word("as")? {
                    self.parse_module_export_name()?
                } else {
                    local.clone()
                };
                specifiers.push(ExportSpecifier { local, exported });
                if !self.is(P::RBrace) {
                    self.expect(P::Comma)?;
                }
            }
            self.next()?;
            let source = if self.eat_word("from")? {
                Some(self.parse_module_specifier()?)
            } else {
                None
            };
            let attributes = if source.is_some() {
                self.parse_import_attributes()?
            } else {
                Vec::new()
            };
            if source.is_none()
                && let Some(pos) = first_reserved
            {
                return self.err("exported local name must be an identifier reference", pos);
            }
            self.semicolon()?;
            return Ok(ExportDecl::Named {
                specifiers,
                source,
                attributes,
                span: self.span_from(start),
            });
        }
        if self.eat_word("default")? {
            let dstart = self.start();
            if self.is_word("function") {
                let f = self.parse_function(dstart, false, true, true)?;
                return Ok(ExportDecl::DefaultDecl(Stmt::Function(std::rc::Rc::new(f))));
            }
            if self.is_word("async") {
                let next = self.peek()?;
                if next.kind == Tok::Ident
                    && next.ident == "function"
                    && !next.escaped
                    && !next.nl_before
                {
                    self.next()?;
                    let f = self.parse_function(dstart, true, true, true)?;
                    return Ok(ExportDecl::DefaultDecl(Stmt::Function(std::rc::Rc::new(f))));
                }
            }
            if self.is_word("class") {
                let c = self.parse_class_with(true, true)?;
                return Ok(ExportDecl::DefaultDecl(Stmt::Class(c)));
            }
            let e = self.parse_expression_allow_in_assignment()?;
            self.semicolon()?;
            return Ok(ExportDecl::DefaultExpr(e, self.span_from(start)));
        }
        // export Declaration / VariableStatement
        let ok = self.tok.kind == Tok::Ident
            && !self.tok.escaped
            && matches!(
                self.tok.ident.as_str(),
                "var" | "let" | "const" | "function" | "class" | "async" | "using" | "await"
            );
        if !ok {
            return self.unexpected();
        }
        let stmt = self.parse_statement_list_item()?;
        match &stmt {
            Stmt::Var(_) | Stmt::Function(_) | Stmt::Class(_) => Ok(ExportDecl::Decl(stmt)),
            _ => self.err("expected a declaration after export", start),
        }
    }

    fn parse_expression_allow_in_assignment(&mut self) -> PResult<Expr> {
        self.with_in(true, |p| p.parse_assignment())
    }
}

trait FromOrComma {
    fn is_from_or_comma(&self) -> bool;
}

impl FromOrComma for crate::lexer::Token {
    /// After `import from`: the default binding is named "from" if what follows is `from` or `,`.
    fn is_from_or_comma(&self) -> bool {
        self.is(P::Comma) || (self.kind == Tok::Ident && !self.escaped && self.ident == "from")
    }
}
