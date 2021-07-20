package main

import (
	"fmt"
	"os"
	"os/exec"
	"time"
)

func main() {
	fmt.Printf("Test package")
}

// StartProxy build and starts the proxy in a docker container.
func StartProxy(dockerfile string, envoy string) error {
	cmd := exec.Command("sh", "-c", fmt.Sprintf("docker build -t proxy -f %s/Dockerfile --build-arg ENVOY_YAML=%s --no-cache . && docker run -d -p 9095:9095 --network envoymesh --name envoy-proxy-test proxy", dockerfile, envoy))
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		fmt.Printf("Error starting proxy container: %v", err)
		return err
	}
	time.Sleep(10 * time.Second)
	return nil
}

// StopProxy stops the container running envoy. Used at the end of each test suite to remove the proxy instance.
func StopProxy() error {
	cmd := exec.Command("sh", "-c", "docker rm -f envoy-proxy-test")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		fmt.Printf("Error removing proxy container: %v", err)
		return err
	}
	return nil
}
