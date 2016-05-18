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
(def ^:const MIGRATIONS_DIR     (str DB_DIR "/migrations"))
(def ^:const SRC_DIR            (.join path (js* "__dirname") ".."))
(def ^:const SRC_MIGRATIONS_DIR (.join path SRC_DIR "migrations"))

(defn setup
  []
  (.sync mkdirp DB_DIR)
  (.sync mkdirp MIGRATIONS_DIR)
  (.sync mkdirp SRC_MIGRATIONS_DIR)
  (.removeSync fs MIGRATIONS_DIR)
  (.copySync fs SRC_MIGRATIONS_DIR MIGRATIONS_DIR))

(defn db-migrate
  [& [env]]
  (.getInstance (node/require "db-migrate")
                true
                (clj->js {:cwd    (if (= env :dev)
                                    SRC_DIR
                                    DB_DIR)
                          :config {:default "sqlite3"
                                   :sqlite3 {:driver "sqlite3"
                                             :filename DB_FILE}}})))

(defn migrate
  "Like node_modules/.bin/db-migrate, but using our programmatically created
   db-migrate instance.

   example usage:

     ews migrate up
     ews migrate create create-user-table"
  []
  (setup)
  (.run (db-migrate :dev)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn- serialize
  "Allows you to use db.serialize, passing in the db as an argument to the
   callback function.

   This allows a serial db operation to be defined as a function before a db is
   instantiated."
  [db f]
  (.serialize db #(f db)))

