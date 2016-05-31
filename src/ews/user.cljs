(ns ews.user
  (:require [ews.config :refer (STATE)]
            [ews.db     :as db]))

(defn get-current-user
  []
  (get STATE "currentUser"))

(defn user
  []
  (if-let [{:keys [name]} (get-current-user)]
    (println "Current user: " name)
    (println "TODO: make user")))
