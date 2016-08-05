(ns ews.db.helpers
  (:require [cljs.nodejs     :as    node]
            [cljs.core.async :refer (chan >! <!)]
            [ews.config      :refer (DB-FILE)])
  (:require-macros [cljs.core.async.macros :refer (go)]))

(defonce sqlite3 (node/require "sqlite3"))

(defn- do-with-ews-db
  "Executes a function `f` as a serial operation using the sqlite db in
   EWS-HOME. `f` takes a single argument, the database."
  [f]
  (let [db (new sqlite3.Database DB-FILE)]
    (.serialize db #(f db))
    (.close db)))

(defn- db-exec
  "Executes SQL `statement` using the sqlite3 db in EWS-HOME.

   If additional `args` are provided, they are interpolated into the SQL."
  [statement & [args]]
  (do-with-ews-db #(.run % statement (apply array (or args [])))))

(defn- db-exec-with-callback
  "Executes SQL statement using the sqlite3 db in EWS-HOME.

   If `args` is non-nil, they are interpolated into the SQL.

   The callback `cb` is passed to the node-sqlite3 db.run function. This
   callback takes a single argument, which will be an error if execution is not
   successful. If execution is successful, then the `this` object will contain
   two properties named `lastID` and `changes` which contain the value of the
   last inserted row ID and the number of rows affected by the SQL statement.
   (God, I hate this.)"
  [statement args cb]
  (do-with-ews-db #(.run % statement (apply array (or args [])) cb)))

(defn- db-exec-returning-last-id
  "Using db-exec-with-callback insanity, executes SQL `statement` using the
   sqlite3 db in EWS-HOME and returns `this.lastID` from the callback.

   (Actually, returns a core.async channel that you can get the lastID from.)

   This can be used to insert a record and get the ID of the new record."
  [statement & [args]]
  (let [c (chan)]
    (db-exec-with-callback statement
                           args
                           #(this-as result
                              (when % (throw %)) ; throw error if not successful
                              (go (>! c (.-lastID result)))))
    c))

(defn- db-get-with-callback
  "Executes SQL `query` using the sqlite3 db in EWS-HOME.

   If `args` is non-nil, they are interpolated into the SQL.

   The callback `cb` is passed to the node-sqlite3 db.get function. This
   callback takes two arguments:

     1. An error, if execution is not successful.
     2. The first row in the results."
  [query args cb]
  (do-with-ews-db #(.get % query (apply array (or args [])) cb)))

(defn- db-get
  "Runs `query` against the sqlite3 db in EWS-HOME and returns the first
   result.

   (Actually, returns a core.async channel from which that result can be
   taken.)

   You can't put nil on a core.async channel, so if the record doesn't exist,
   the channel will yield an empty map."
  [query & [args]]
  (let [c (chan)]
    (db-get-with-callback query args (fn [err row]
                                       (when err (throw err))
                                       (go (>! c (if row
                                                   (js->clj row)
                                                   {})))))
    c))

