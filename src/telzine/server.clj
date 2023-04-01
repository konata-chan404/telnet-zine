(ns telzine.server
  (:gen-class))

(import '(java.net ServerSocket)
        '(java.io BufferedReader InputStreamReader OutputStreamWriter))

(defn handle-client [socket handler]
  (let [in (BufferedReader. (InputStreamReader. (.getInputStream socket)))
        out (OutputStreamWriter. (.getOutputStream socket))]
    (handler in out)))

(defn start-server [port handler]
  (let [server (ServerSocket. port)]
    (println (str "Telnet server started on port " port))
    (while true
      (let [socket (.accept server)]
        (future
          (try
            (handle-client socket handler)
            (catch Exception e
              (let [msg (.getMessage e)]
                (when msg
                  (println msg)))
              (.close socket))))))))