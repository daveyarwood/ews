(ns ews.cli
  (:require [cljs.nodejs :as node]
            [ews.db      :as db]
            [ews.fs      :as fs]))

(node/enable-util-print!)

(defn -main [& args]
  (let [fname (first args)
        db-file (if (fs/exists? fname) fname "test-db.sqlite3")]
    (db/test-sqlite3 db-file)))

(set! *main-cli-fn* -main)
