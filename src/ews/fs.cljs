(ns ews.fs
  (:require [cljs.nodejs       :as node]
            [cognitect.transit :as t])
  (:refer-clojure :exclude (exists?)))

(defonce fs (node/require "fs"))

(defn exists? [filename]
  (try
    (.statSync fs filename)
    true
    (catch js/Error e
      false)))

(defn read-json-file
  "Reads a UTF-8 file containing JSON and parses it using transit-cljs.

   Returns a ClojureScript data structure."
  [filename]
  (let [rdr (t/reader :json)]
    (t/read rdr (.readFileSync fs filename "utf-8"))))
