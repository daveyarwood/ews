(ns ews.db
  (:require [ews.db.helpers :refer (db-exec-returning-last-id
                                    db-get)]))

(defn create-user!
  "Creates a user and returns a core.async channel from which you can take the
   ID of the new record."
  [{:keys [name] :as user}]
  (db-exec-returning-last-id "INSERT INTO ews_user (name) VALUES (?)" [name]))

(defn user
  "Gets a user from the database.

   Returns a core.async channel from which the user can be taken."
  [id]
  (db-get "SELECT * FROM ews_user WHERE id = ?" [id]))

