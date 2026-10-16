package dev.kahvi.reactnative

import android.content.Context
import com.facebook.jni.HybridData
import com.facebook.jni.annotations.DoNotStrip
import com.facebook.jni.annotations.DoNotStripAny
import com.facebook.react.runtime.JSRuntimeFactory
import com.facebook.soloader.SoLoader
import java.io.File

/**
 * React Native's JS engine as kahvi: pass it to `getDefaultReactHost`.
 *
 * With a [context], kahvi keeps what parsing the bundle learned under the
 * app's code cache directory, so later runtimes (the worklets UI runtime) and
 * later launches skip that work. Android clears the directory when the app is
 * updated.
 */
@DoNotStripAny
public class KahviRuntimeFactory(context: Context? = null) : JSRuntimeFactory(initHybrid()) {
  init {
    if (context != null) {
      setCodeCacheDir(File(context.codeCacheDir, "kahvi").path)
    }
  }

  private companion object {
    init {
      SoLoader.loadLibrary("kahviruntimefactory")
    }

    @DoNotStrip @JvmStatic private external fun initHybrid(): HybridData

    @DoNotStrip @JvmStatic private external fun setCodeCacheDir(path: String)
  }
}
