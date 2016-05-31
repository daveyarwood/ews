(ns ews.config
  (:require [cljs.nodejs :as node]))

(defonce expand-home-dir (node/require "expand-home-dir"))
(defonce mkdirp          (node/require "mkdirp"))

(def ^:const EWS-HOME (expand-home-dir "~/.ews"))

; ensure that EWS-HOME exists
(.sync mkdirp EWS-HOME)

(def ^:const STATE "TODO: read state.json file")
