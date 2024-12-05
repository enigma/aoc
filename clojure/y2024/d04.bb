#!/usr/bin/env bb

(ns y2024.d04
  (:require [babashka.cli :as cli]
            [clojure.string :as str]))


(def cli-options {:input {:default "../../inputs/2024/04.input" :type :string}})

(def opts (cli/parse-opts *command-line-args* {:spec cli-options}))

(def input
  (->> (slurp (get opts :input))))

(defn part1 [data]
  (let [w (str/index-of data "\n")
        patterns
        (flatten (for [i [0 w (inc w) (dec w)]]
                   [(re-pattern (str "(?s)(?=X.{" i "}M.{" i "}A.{" i "}S)"))
                    (re-pattern (str "(?s)(?=S.{" i "}A.{" i "}M.{" i "}X)"))]))]
    (reduce + (map #(count (re-seq % data)) patterns))))


(defn part2 [data]
  (let [w (dec (str/index-of data "\n"))
        patterns
        (for [[tl tr bl br] ["MMSS" "MSMS" "SSMM" "SMSM"]]
          (re-pattern (str "(?s)(?=" tl "." tr ".{" w "}A.{" w "}" bl "." br")")))]
    (reduce + (map #(count (re-seq % data)) patterns))))

(prn {:part1 (part1 input)
      :part2 (part2 input)})
