(ns telzine.core
  (:gen-class)
  (:require [telzine.server :as server]))

(defn handle-line [in out line]
  (case line
    "clear-screen"
    (do (.write out "\033[2J\033[H")
        (.flush out))
    "quit"
    (throw (ex-info "Client disconnected." {}))
    (:default
     (.write out (str "Unknown command: " line "\n"))
     (.flush out))))

(defn -main []
  (server/start-server 2323 handle-line))