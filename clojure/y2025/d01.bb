#!/usr/bin/env bb

(ns y2025.d01
  (:require [clojure.string :as str]))

(defn parse [s]
  (->> s
       str/split-lines
       (map #(let [sign (if (= (first %1) \L) -1 1)
                   value (parse-long (subs %1 1))]
               (* sign value)))
       (into [])))

(defn part1 [data]
  (->> data
       (reductions #(mod (+ %1 %2) 100) 50)
       rest  ; drop the initial element
       (filter zero?)
       count))

(defn part2 [data]
  (->> data
       (reductions
        (fn [[cur _] d]
          [(mod (+ cur d) 100)
           (cond
             (>= d 0)                       (quot (+ cur d) 100)
             (and (< 0 cur) (<= cur (- d))) (inc (quot (- (- d) cur) 100))
             :else                          (quot (- d) 100))])
        [50 0])
       (map second)
       (apply +)))

(when (= *file* (System/getProperty "babashka.file"))
  (let [input (->> *command-line-args* last slurp parse)
        res {:part1 (part1 input)
             :part2 (part2 input)}]
    (prn res)))