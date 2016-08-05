(ns ews.cli
  (:require [cljs.nodejs    :as node]
            [clojure.string :as str]
            [ews.db.migrate :as db]
            [ews.fs         :as fs]
            [ews.user       :as user]))

(node/enable-util-print!)

(def commands
  {"help"
   {:help    "Displays available ews commands."
    :action  (fn
               ([]
                (->> (for [[cmd {:keys [help]}] commands
                           :when help]
                       (str cmd \tab help))
                     (str/join "\n")
                     println))
               ([cmd]
                (if-let [{:keys [help subcmds]} (get commands cmd)]
                  (let [cmd-help     (when help (str cmd \tab help))
                        subcmds-help (for [[subcmd {:keys [help]}] subcmds
                                           :when help]
                                       (str cmd \space subcmd \tab help))]
                    (println (str cmd-help
                                  (when-not (empty? subcmds-help)
                                    (apply str \newline
                                           \newline
                                           subcmds-help)))))
                  (do
                    (println "Unrecognized command.")
                    (.exit node/process 1)))))
    :subcmds {"<cmd>"
              {:help "Displays help about a particular command."}}}

   "setup"
   {:help   "Bootstraps the ews database file."
    :action #(db/setup)}

   ; for development only - not shown in help text
   "migrate"
   {:action #(db/migrate)}

   "user"
   {:help   "Displays information about the current user."
    :action #(user/user)}})

(defn -main [cmd & args]
  (if-let [{:keys [action]} (get commands cmd)]
    (apply action args)
    (do
      (println "Unrecognized command.")
      (.exit node/process 1))))

(set! *main-cli-fn* -main)
