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
        try {
          await init();
          const res = await refetch(event.request);
          return res;
        } catch (error) {
          console.log("[SW] Fetch error:", error);
          return;
        }
      })()
    );
  }
});
