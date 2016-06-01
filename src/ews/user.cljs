(ns ews.user
  (:require [ews.config :as config :refer (get-state assoc-state!)]
            [ews.db     :as db]))

(defn prompt-for-user-info!
  []
  "TODO")

(defn create-user!
  "Prompts the user for information, creates a user in the database, sets the
   new user as the current user, and returns that user."
  []
  (let [new-user (prompt-for-user-info!)
        db-user  (db/create-user! new-user)]
    (assoc-state! "currentUser" (select-keys db-user [:id]))
    db-user))

(defn current-user!
  "Returns the current user, if it exists in the state file.

   If it doesn't, creates a user by prompting for information, sets the new
   user as the current user in the state file, and returns that user."
  []
  (or (get-state "currentUser")
      (create-user!)))

(defn user
  []
  (if-let [{:keys [id]} (current-user!)]
    (println "Current user: " id)
    (println "TODO: make user")))
