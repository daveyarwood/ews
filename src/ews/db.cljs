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

(defn migrate
  "TODO (using db-migrate node package)"
  [])
