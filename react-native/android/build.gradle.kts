// The kahvi engine for React Native's new architecture on Android, built like
// @react-native-community/javascriptcore's engine package.
import com.android.build.gradle.internal.tasks.factory.dependsOn
import org.apache.tools.ant.taskdefs.condition.Os

val reactNativeDir = findNodePackageDir("react-native")
val prefabHeadersDir = file("${layout.buildDirectory.get()}/prefab-headers")

if (findProperty("hermesEnabled") == "true") {
  throw GradleException(
    "Set hermesEnabled=false in gradle.properties: with Hermes the bundle is compiled to Hermes bytecode.\n"
  )
}

plugins {
  id("com.android.library")
  id("org.jetbrains.kotlin.android")
}

android {
  namespace = "dev.kahvi.reactnative"
  compileSdk = safeExtGet("compileSdkVersion", 35)
  if (rootProject.hasProperty("ndkPath")) {
    ndkPath = rootProject.extra["ndkPath"].toString()
  }
  if (rootProject.hasProperty("ndkVersion")) {
    ndkVersion = rootProject.extra["ndkVersion"].toString()
  }

  defaultConfig {
    minSdk = safeExtGet("minSdkVersion", 24)
    @Suppress("UnstableApiUsage")
    externalNativeBuild.cmake {
      arguments += listOf(
        "-DANDROID_STL=c++_shared",
        "-DREACT_NATIVE_DIR=${reactNativeDir.toPlatformString()}",
      )
      targets("kahviruntimefactory")
      abiFilters(*reactNativeArchitectures().toTypedArray())
    }
  }

  externalNativeBuild {
    cmake {
      path("CMakeLists.txt")
    }
  }

  packaging {
    jniLibs {
      excludes.add("**/libc++_shared.so")
      excludes.add("**/libfbjni.so")
      excludes.add("**/libjsi.so")
      excludes.add("**/libreactnative.so")
      pickFirsts.add("**/libkahviruntimefactory.so")
    }
  }

  buildFeatures {
    prefab = true
    prefabPublishing = true
  }

  // Libraries that make their own runtimes (react-native-worklets) link
  // kahviruntimefactory and call kahvi::makeKahviRuntime (KahviJSI.h).
  prefab {
    register("kahviruntimefactory") {
      headers = file(prefabHeadersDir).absolutePath
    }
  }
}

dependencies {
  implementation("com.facebook.yoga:proguard-annotations:1.19.0")
  compileOnly("com.facebook.fbjni:fbjni:0.7.0")
  implementation("com.facebook.react:react-android")
}

val createPrefabHeadersDir by
  tasks.registering(Copy::class) {
    from("../cpp")
    from("../include")
    include("*.h")
    into(prefabHeadersDir)
  }

tasks.named("preBuild").dependsOn(createPrefabHeadersDir)

private fun findNodePackageDir(packageName: String): File {
  val nodeCommand = listOf("node", "--print", "require.resolve('${packageName}/package.json')")
  val proc = ProcessBuilder(nodeCommand).directory(rootDir).start()
  val error = proc.errorStream.bufferedReader().readText()
  if (error.isNotEmpty()) {
    throw GradleException("findNodePackageDir(${packageName}) failed:\n$error")
  }
  return File(proc.inputStream.bufferedReader().readText().trim()).parentFile.absoluteFile
}

private fun File.toPlatformString(): String {
  var result = path.toString()
  if (Os.isFamily(Os.FAMILY_WINDOWS)) {
    result = result.replace(File.separatorChar, '/')
  }
  return result
}

private fun reactNativeArchitectures(): List<String> {
  val value = findProperty("reactNativeArchitectures")
  return value?.toString()?.split(",") ?: listOf("armeabi-v7a", "x86", "x86_64", "arm64-v8a")
}

private fun <T> safeExtGet(prop: String, fallback: T): T {
  @Suppress("UNCHECKED_CAST")
  return rootProject.extra[prop] as? T ?: fallback
}
