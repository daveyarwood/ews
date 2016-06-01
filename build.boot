(set-env!
  :source-paths #{"src"}
  :dependencies '[[org.clojure/clojurescript  "1.7.228"]
                  [com.cognitect/transit-cljs "0.8.237"]
                  [adzerk/boot-cljs           "1.7.228-1" :scope "test"]])

(require '[adzerk.boot-cljs :refer :all])

(task-options!
  cljs {:compiler-options {:target        :nodejs
                           :optimizations :simple}})

(deftask build
  []
  (comp
    (cljs)
    (target)
    (with-pass-thru fs
      (dosh "npm" "link"))))

(deftask dev
  []
  (comp
    (watch)
    (build)))
