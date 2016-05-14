(set-env!
  :source-paths   #{"src"}
  :resource-paths #{"target/js"}
  :dependencies   '[[org.clojure/clojurescript "1.7.228"]
                    [adzerk/boot-cljs          "1.7.228-1" :scope "test"]])

(require '[adzerk.boot-cljs :refer :all])

(def +version+ "0.0.1")

(task-options!
  pom {:project     'daveyarwood/ews
       :version     +version+
       :description "electronic worksheet system"
       :url         "https://github.com/daveyarwood/ews"
       :scm         {:url "https://github.com/daveyarwood/ews"}
       :license     {"name" "Eclipse Public License"
                     "url" "http://www.eclipse.org/legal/epl-v10.html"}}
  jar {:main 'ews.Nashorn})

(deftask build
  []
  (comp
    (cljs)
    (javac)
    (pom)
    (uber)
    (jar)
    (target)))
