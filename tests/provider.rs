use tf_bindgen::{app::App, stack::Stack};
use tf_kubernetes::kubernetes::resource::kubernetes_namespace::*;
use tf_kubernetes::kubernetes::Provider;

#[test]
fn default_provider() {
    let app = App::new();
    let stack = Stack::new(&app, "default_provider");

    Provider::create(&stack).build();

    KubernetesNamespace::create(&stack, "default")
        .metadata(KubernetesNamespaceMetadata::builder().build())
        .build();

    app.validate(true).unwrap();
}

#[test]
fn config_path_provider() {
    let app = App::new();
    let stack = Stack::new(&app, "config_path_provider");

    Provider::create(&stack)
        .config_path("~/.kube/config")
        .build();

    KubernetesNamespace::create(&stack, "default")
        .metadata(KubernetesNamespaceMetadata::builder().build())
        .build();

    app.validate(true).unwrap();
}
