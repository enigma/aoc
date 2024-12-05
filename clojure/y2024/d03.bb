#!/usr/bin/env bb

(ns y2024.d03
  (:require [babashka.cli :as cli]))


(def cli-options {:input {:default "../../inputs/2024/03.input" :type :string}})

(def opts (cli/parse-opts *command-line-args* {:spec cli-options}))

(def input
  (->> (slurp (get opts :input))))

(defn solve [data part2?]
  (let [re #"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"]
    (loop [matches (re-seq re data)
           result 0
           doing? true]
      (if (empty? matches)
        result
        (let [[match & rst] (first matches)
              [res do?]
              (cond
                (= match "do()") [result true]
                (= match "don't()") [result false]

                (or doing?  (not part2?))
                (let [[a b] (map parse-long rst)]
                  [(+ result (* a b)) doing?])

                :else
                [result doing?])]
          (recur (rest matches) res do?))))))

(defn part1 [data]
  (solve data false))


(defn part2 [data]
  (solve data true))

(prn {:part1 (part1 input)
      :part2 (part2 input)})
