(ns ews.fs
  (:require [cljs.nodejs :as node])
  (:refer-clojure :exclude (exists?)))

(defonce fs (node/require "fs"))

(defn exists? [filename]
  (try
    (.statSync fs filename)
    true
    (catch js/Error e
      false)))

