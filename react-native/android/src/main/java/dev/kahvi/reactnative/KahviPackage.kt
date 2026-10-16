package dev.kahvi.reactnative

import com.facebook.react.ReactPackage
import com.facebook.react.bridge.NativeModule
import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.uimanager.ViewManager

/**
 * No modules or views: it exists so that autolinking (which skips Android
 * libraries without a ReactPackage) adds this Gradle project to the app, for
 * KahviRuntimeFactory and for react-native-worklets' prefab dependency.
 */
public class KahviPackage : ReactPackage {
  override fun createNativeModules(reactContext: ReactApplicationContext): List<NativeModule> =
    emptyList()

  override fun createViewManagers(reactContext: ReactApplicationContext): List<ViewManager<*, *>> =
    emptyList()
}
