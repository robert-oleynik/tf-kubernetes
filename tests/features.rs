use tf_bindgen::Stack;
use tf_kubernetes::resource::kubernetes_pod;

#[test]
fn resource_multi_blocks() {
    let stack = Stack::new("test-multi-blocks");
    let pod = tf_bindgen::codegen::resource! {
        &stack, resource "kubernetes_pod" "test" {
            metadata {
                name = "nginx"
            }
            spec {
                container {
                    image = "nginx"
                    name = "nginx"
                    port {
                        container_port = 80
                    }
                    port {
                        container_port = 443
                    }
                }
            }
        }
    };
    assert_eq!(80, *pod.spec[0].container[0].port[0].container_port.get());
    assert_eq!(443, *pod.spec[0].container[0].port[1].container_port.get());
}
