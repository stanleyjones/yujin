import init, { refetch } from "./pkg/yujin.js";

self.addEventListener("install", () => {
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(clients.claim());
});

self.addEventListener("fetch", (event) => {
  if (event.request.headers.has("X-Many-Request")) {
    event.respondWith(
      (async () => {
        await init();
        return refetch(event.request);
      })()
    );
  }
});
