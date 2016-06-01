(ns ews.config
  (:require [cljs.nodejs :as node]
            [ews.fs      :as fs]))

(defonce expand-home-dir (node/require "expand-home-dir"))
(defonce fs              (node/require "fs"))
(defonce mkdirp          (node/require "mkdirp"))

(def ^:const EWS-HOME (expand-home-dir "~/.ews"))

; ensure that EWS-HOME exists
(.sync mkdirp EWS-HOME)

(def ^:const STATE
  (let [state-file (str EWS-HOME "/state.json")
        _          (when-not (fs/exists? state-file)
                     (.writeFileSync fs state-file "{}"))]
    (fs/read-json-file state-file)))

(defn get-state
  [k]
  (get STATE k))

(defn assoc-state!
  [k v]
  "TODO")
