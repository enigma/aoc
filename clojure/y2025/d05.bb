#!/usr/bin/env bb

(ns y2025.d05
  (:require [babashka.cli :as cli]
            [clojure.string :as str]))


(defn parse [fname]
  (let [[sranges sings] (str/split (-> fname slurp str/trim) #"\n\n")]
    [(->> sranges str/split-lines (mapv #(str/split % #"-")) (mapv #(mapv parse-long %)) sort)
     (->> sings str/split-lines (map parse-long))]))
        
(defn merged-ranges [ranges]
  (reduce
    (fn [merged [a b]]
     (let [[oa ob] (last merged)]
        (cond
          (<= b ob) merged
          (> a ob) (conj merged [a b])
          :else (conj (pop merged) [oa (max ob b)]))
       ))
    [(first ranges)]
    (rest ranges)))

(defn part1 [[ranges ings]]
  (apply + (for [i ings] (if (some (fn [[a b]] (<= a i b)) ranges)  1 0))))

(defn part2 [[ranges _ings]]
  (apply + (mapv (fn [[a b]] (+ 1 b (- a))) (merged-ranges ranges))))

(prn (let [input (->> *command-line-args* last parse)]
       {:part1 (part1 input)
        :part2 (part2 input)}))
