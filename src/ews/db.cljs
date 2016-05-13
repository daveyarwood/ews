(ns ews.db
  (:require [cljs.nodejs :as node]))

(defonce sqlite3 (node/require "sqlite3"))

; example from node-sqlite3 readme
; (this is a literal js translation, super ugly cljs code)
(defn test-sqlite3 []
  (let [db   (new sqlite3.Database ":memory:")]
    (.serialize db
      (fn []
        (.run db "CREATE TABLE lorem (info TEXT)")
        (let [stmt (.prepare db "INSERT INTO lorem VALUES (?)")]
          (dotimes [n 10]
            (.run stmt (str "Ipsum " n)))
          (.finalize stmt))
        (.each db "SELECT rowid AS id, info FROM lorem"
          (fn [e row]
            (js/console.log (str (.-id row) ": " (.-info row)))))))
    (.close db)))

