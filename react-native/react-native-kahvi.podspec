# The kahvi engine for React Native's new architecture on iOS, built like
# @react-native-community/javascriptcore's React-jsc. Run
# scripts/build-apple.sh first for apple/KahviFFI.xcframework.
require "json"

package = JSON.parse(File.read(File.join(__dir__, "package.json")))

folly_config = get_folly_config()
boost_config = get_boost_config()

Pod::Spec.new do |s|
  s.name                = "react-native-kahvi"
  s.version             = package["version"]
  s.summary             = package["description"]
  s.homepage            = "https://github.com/mozzius/kahvi"
  s.license             = package["license"]
  s.author              = "kahvi"
  s.platforms           = { :ios => "15.1" }
  s.source              = { :git => "https://github.com/mozzius/kahvi.git", :tag => "#{s.version}" }
  s.source_files        = "cpp/*.{cpp,h}", "apple/*.{mm,h}", "include/kahvi.h"
  s.public_header_files = "apple/RCTKahviInstanceFactory.h"
  s.vendored_frameworks = "apple/KahviFFI.xcframework"
  s.compiler_flags      = folly_config[:compiler_flags] + " " + boost_config[:compiler_flags]
  s.pod_target_xcconfig = {
    "DEFINES_MODULE" => "YES",
    "CLANG_CXX_LANGUAGE_STANDARD" => "c++20",
  }
  s.module_name         = "ReactKahvi"

  s.dependency "RCT-Folly", folly_config[:version]
  s.dependency "React-jsitooling"
  s.dependency "React-cxxreact"
  s.dependency "React-jsi"
end
