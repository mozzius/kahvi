package dev.kahvi.reactnative

import com.facebook.jni.HybridData
import com.facebook.jni.annotations.DoNotStrip
import com.facebook.jni.annotations.DoNotStripAny
import com.facebook.react.runtime.JSRuntimeFactory
import com.facebook.soloader.SoLoader

/** React Native's JS engine as kahvi: pass it to `getDefaultReactHost`. */
@DoNotStripAny
public class KahviRuntimeFactory : JSRuntimeFactory(initHybrid()) {
  private companion object {
    init {
      SoLoader.loadLibrary("kahviruntimefactory")
    }

    @DoNotStrip @JvmStatic private external fun initHybrid(): HybridData
  }
}
