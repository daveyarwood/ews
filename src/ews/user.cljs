(ns ews.user
  (:require [cljs.nodejs     :as node]
            [cljs.core.async :refer (<! chan >!)]
            [ews.config      :as config :refer (get-state assoc-state!)]
            [ews.db          :as db])
  (:require-macros [cljs.core.async.macros :refer (go)]))

(defonce prompt ((node/require "prompt-sync")))

(defn prompt-for-user-info!
  []
  {:name (prompt "Please enter your name: ")})

(defn create-user!
  "Prompts the user for information, creates a user in the database, sets the
   new user as the current user, and returns the ID of the new record."
  []
  (go
    (let [new-user (prompt-for-user-info!)
          id       (<! (db/create-user! new-user))]
      (assoc-state! "currentUser" {:id id})
      id)))

(defn current-user!
  "Returns a core.async channel from which the current user may be taken.

   If the current user exists in the state file, the channel will yield that
   user.

   If it doesn't, creates a user by prompting for information, sets the new
   user as the current user in the state file, and returns that user."
  []
  (let [c (chan)]
    (go
      (>! c (let [id (or (some-> (get-state "currentUser") :id)
                         (do
                           (println "No users have been created yet."
                                    "Let's create one now")
                           (<! (create-user!))))]
              (<! (db/user id)))))
    c))

(defn user
  []
  (go
    (let [{:strs [id name] :as user} (<! (current-user!))]
      (println "Current user: " name))))
