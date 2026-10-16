// Expo config plugin: builds the app with kahvi instead of Hermes.
//   plugins: ["react-native-kahvi"]
// Android: turns Hermes off and passes KahviRuntimeFactory to
// ExpoReactHostFactory.getDefaultReactHost in MainApplication.kt (which needs
// patches/expo+57.0.24.patch to honour it).
const { withGradleProperties, withMainApplication } = require("expo/config-plugins");

const IMPORT = "import dev.kahvi.reactnative.KahviRuntimeFactory";

function withKahviGradleProperties(config) {
  return withGradleProperties(config, (config) => {
    const props = config.modResults.filter((p) => !(p.type === "property" && p.key === "hermesEnabled"));
    props.push({ type: "property", key: "hermesEnabled", value: "false" });
    config.modResults = props;
    return config;
  });
}

function withKahviMainApplication(config) {
  return withMainApplication(config, (config) => {
    if (config.modResults.language !== "kt") {
      throw new Error("react-native-kahvi: MainApplication must be Kotlin");
    }
    let src = config.modResults.contents;
    if (!src.includes(IMPORT)) {
      src = src.replace(/^(package [^\n]+\n)/m, `$1\n${IMPORT}\n`);
    }
    // The context lets kahvi keep its parse cache in the app's code cache dir.
    src = src.replace("KahviRuntimeFactory()", "KahviRuntimeFactory(applicationContext)");
    if (!src.includes("KahviRuntimeFactory(applicationContext)")) {
      const call = "ExpoReactHostFactory.getDefaultReactHost(";
      if (!src.includes(call)) {
        throw new Error("react-native-kahvi: no ExpoReactHostFactory.getDefaultReactHost call in MainApplication.kt");
      }
      src = src.replace(call, `${call}\n      jsRuntimeFactory = KahviRuntimeFactory(applicationContext),`);
    }
    config.modResults.contents = src;
    return config;
  });
}

module.exports = function withKahvi(config) {
  return withKahviMainApplication(withKahviGradleProperties(config));
};
