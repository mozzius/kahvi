#pragma once

#import <react/runtime/JSRuntimeFactoryCAPI.h>

#ifdef __cplusplus
extern "C" {
#endif

/// React Native's JS engine as kahvi: return it from
/// RCTDefaultReactNativeFactoryDelegate's createJSRuntimeFactory.
JSRuntimeFactoryRef jsrt_create_kahvi_factory(void);

#ifdef __cplusplus
}
#endif
