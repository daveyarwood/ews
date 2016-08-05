(ns ews.fs
  (:require [cljs.nodejs :as    node]
            [ews.util    :refer (from-json)])
  (:refer-clojure :exclude (exists?)))

(defonce fs (node/require "fs"))

(defn exists? [filename]
  (try
    (.statSync fs filename)
    true
    (catch js/Error e
      false)))

(defn read-json-file
  "Reads a UTF-8 file containing JSON and parses it using transit.

   Returns a ClojureScript data structure."
  [filename]
  (from-json (.readFileSync fs filename "utf-8")))
