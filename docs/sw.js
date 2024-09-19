// Register the service worker
if ("serviceWorker" in navigator) {
  navigator.serviceWorker
    .register("service-worker.js")
    .then((registration) => {
      console.log("Service worker registado:", registration);
    })
    .catch((error) => {
      console.error("Service worker registration failed:", error);
    });
}
