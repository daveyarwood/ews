(ns ews.db
  (:require [cljs.nodejs     :as    node]
            [cljs.core.async :refer (chan >! <!)]
            [ews.config      :refer (EWS-HOME)])
  (:require-macros [cljs.core.async.macros :refer (go)]))

(node/enable-util-print!)

(defonce fs              (node/require "node-fs-extra"))
(defonce path            (node/require "path"))
(defonce mkdirp          (node/require "mkdirp"))
(defonce sqlite3         (node/require "sqlite3"))

(def ^:const DB-FILE            (str EWS-HOME "/ews.db"))
(def ^:const SRC-DIR            (.join path (js* "__dirname") ".."))
(def ^:const SRC-MIGRATIONS-DIR (.join path SRC-DIR "migrations"))

; ensure that /usr/local/lib/node_modules/ews/migrations exists
(.sync mkdirp SRC-MIGRATIONS-DIR)

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

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

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

   This can be used to insert a record and get the ID of the new record."
  [statement & [args]]
  (let [c (chan)
        a (atom nil)]
    (db-exec-with-callback statement
                           args
                           #(this-as result
                              (when % (throw %)) ; throw error if not successful
                              (prn :outside :lastID (.-lastID result))
                              (go (>! c (do (prn :inside :lastID (.-lastID result))
                                            (.-lastID result))))))
    (go (reset! a (<! c)))
    @a))

(defn create-user!
  [{:keys [name] :as user}]
  (let [v (db-exec-returning-last-id "INSERT INTO ews_user (name) VALUES (?)" [name])]
    (prn :got v)
    v))

