#import "RCTKahviInstanceFactory.h"

#include "KahviInstance.h"

extern "C" {

JSRuntimeFactoryRef jsrt_create_kahvi_factory(void)
{
  // js_runtime_factory_destroy deletes it as a facebook::react::JSRuntimeFactory.
  facebook::react::JSRuntimeFactory *factory = new kahvi::KahviRuntimeFactory();
  return reinterpret_cast<JSRuntimeFactoryRef>(factory);
}

} // extern "C"
