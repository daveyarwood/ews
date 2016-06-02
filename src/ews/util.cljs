(ns ews.util
  (:require [cognitect.transit :as t]))

(defn from-json
  "Reads a ClojureScript expression (via transit) from a JSON string."
  [x]
  (let [r (t/reader :json)]
    (t/read r x)))

(defn to-json
  "Serializes a ClojureScript expression to a JSON string."
  [x]
  (let [w (t/writer :json)]
    (t/write w x)))

