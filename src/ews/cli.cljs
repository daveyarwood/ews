(ns ews.cli
  (:require [cljs.nodejs    :as node]
            [ews.db.migrate :as db]
            [ews.fs         :as fs]
            [ews.user       :as user]))

(node/enable-util-print!)

(defn -main [cmd & args]
  ; pass all args after `cmd` to the subcommand
  ; e.g. args after `ews migrate` are passed to the `db-migrate` process
  (.shift (.-argv node/process))

  (case cmd
    "setup"   (db/setup)
    "migrate" (db/migrate)
    "user"    (user/user)
    (do
      (println "Unrecognized command.")
      (.exit node/process 1))))

(set! *main-cli-fn* -main)
