(ns ews.db
  (:require [cljs.nodejs :as node]))

(defonce sqlite3 (node/require "sqlite3"))

(defn- serialize
  "Allows you to use db.serialize, passing in the db as an argument to the
   callback function.

   This allows a serial db operation to be defined as a function before a db is
   instantiated."
  [db f]
  (.serialize db #(f db)))

; example from node-sqlite3 readme
; (this is a literal js translation, super ugly cljs code)
(defn test-sqlite3 [& [db-file]]
  (let [test-db (fn [db]
                  (.run db "CREATE TABLE lorem (info TEXT)")
                  (let [stmt (.prepare db "INSERT INTO lorem VALUES (?)")]
                    (dotimes [n 10]
                      (.run stmt (str "Ipsum " n)))
                    (.finalize stmt))
                  (.each db "SELECT rowid AS id, info FROM lorem"
                    (fn [e row]
                      (js/console.log (str (.-id row) ": " (.-info row)))))) ]
    (doto (new sqlite3.Database (or db-file ":memory:"))
      (serialize test-db)
      (.close))))

