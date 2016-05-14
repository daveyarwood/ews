(ns ews.cli)

(defn -main [& args]
  (println "hello from ews.cli"))

(set! *main-cli-fn* -main)
