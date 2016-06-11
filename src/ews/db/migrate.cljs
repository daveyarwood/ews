(ns ews.db.migrate
  (:require [cljs.nodejs :as    node]
            [ews.config  :refer (SRC-DIR DB-FILE)]))

(defn db-migrate
  []
  (.getInstance (node/require "db-migrate")
                true
                (clj->js {:cwd    SRC-DIR
                          :config {:default "sqlite3"
                                   :sqlite3 {:driver "sqlite3"
                                             :filename DB-FILE}}})))

(defn setup
  "Bootstraps ews.db:

     - creates it if it doesn't exist
     - runs any migrations that haven't been run yet"
  []
  (.up (db-migrate)))

(defn migrate
  "Like node_modules/.bin/db-migrate, but using our programmatically created
   db-migrate instance.

   example usage:

     ews migrate up
     ews migrate create create-user-table"
  []
  (.run (db-migrate)))

