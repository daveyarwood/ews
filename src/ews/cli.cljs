(ns ews.cli
  (:require [cljs.nodejs :as node]
            [ews.db      :as db]
            [ews.fs      :as fs]))

(node/enable-util-print!)

(defn -main [& args]
  (db/test-sqlite3)
  #_(let [fname (first args)]
    (if (fs/exists? fname)
      (println "file exists")
      (println "file does not exist"))))

(set! *main-cli-fn* -main)
