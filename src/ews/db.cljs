(ns ews.db
  (:require [cljs.nodejs :as node]))

(node/enable-util-print!)

(defonce fs              (node/require "node-fs-extra"))
(defonce path            (node/require "path"))
(defonce expand-home-dir (node/require "expand-home-dir"))
(defonce mkdirp          (node/require "mkdirp"))
(defonce sqlite3         (node/require "sqlite3"))

(def ^:const DB_DIR             (expand-home-dir "~/.ews"))
(def ^:const DB_FILE            (str DB_DIR "/ews.db"))
(def ^:const SRC_DIR            (.join path (js* "__dirname") ".."))
(def ^:const SRC_MIGRATIONS_DIR (.join path SRC_DIR "migrations"))

; ensure that ~/.ews and /usr/local/lib/node_modules/ews/migrations exist
(.sync mkdirp DB_DIR)
(.sync mkdirp SRC_MIGRATIONS_DIR)

(defn db-migrate
  []
  (.getInstance (node/require "db-migrate")
                true
                (clj->js {:cwd    SRC_DIR
                          :config {:default "sqlite3"
                                   :sqlite3 {:driver "sqlite3"
                                             :filename DB_FILE}}})))

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

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn- serialize
  "Allows you to use db.serialize, passing in the db as an argument to the
   callback function.

   This allows a serial db operation to be defined as a function before a db is
   instantiated."
  [db f]
  (.serialize db #(f db)))

