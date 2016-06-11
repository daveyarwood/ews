(ns ews.config
  (:require [cljs.nodejs :as    node]
            [ews.fs      :as    fs]
            [ews.util    :refer (to-json)]))

(defonce expand-home-dir (node/require "expand-home-dir"))
(defonce fs              (node/require "fs"))
(defonce path            (node/require "path"))
(defonce mkdirp          (node/require "mkdirp"))

(def ^:const EWS-HOME (expand-home-dir "~/.ews"))

; ensure that EWS-HOME exists
(.sync mkdirp EWS-HOME)

(def ^:const DB-FILE            (str EWS-HOME "/ews.db"))
(def ^:const SRC-DIR            (.join path (js* "__dirname") ".."))
(def ^:const SRC-MIGRATIONS-DIR (.join path SRC-DIR "migrations"))

; ensure that /usr/local/lib/node_modules/ews/migrations exists
(.sync mkdirp SRC-MIGRATIONS-DIR)

(def ^:const STATE-FILE (str EWS-HOME "/state.json"))

; if state file doesn't exist, create it and initialize it with an empty JSON
; object
(when-not (fs/exists? STATE-FILE)
  (.writeFileSync fs STATE-FILE "{}"))

(def ^:const STATE (fs/read-json-file STATE-FILE))

(defn get-state
  [k]
  (get STATE k))

(defn write-state!
  [new-state]
  (.writeFileSync fs STATE-FILE (to-json new-state)))

(defn assoc-state!
  [k v]
  (write-state! (assoc STATE k v)))

(defn update-state!
  [k f & args]
  (write-state! (apply update STATE k f args)))
