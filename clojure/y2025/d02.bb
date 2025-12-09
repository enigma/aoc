#!/usr/bin/env bb

(ns y2025.d02
  (:require [clojure.string :as str]))


(defn merge-ranges [rs]
  (reduce
   (fn [[[oa ob] & rmerged :as merged] [a b]]
     (cond
       (<= a ob)  (conj rmerged [oa (max b ob)])
       :else (conj merged [a b])))
   [(first rs)]
   (rest rs)))


(defn parse [s]
  (->> (str/split (str/trim s) #",")
       (mapv #(str/split % #"-"))
       (mapv #(mapv parse-long %))
       sort
       (into [])))


(defn merge [a b]
  (let [[ah & as] a
        [bh & bs] b]
    (cond
      (empty? a) b
      (empty? b) a
      (= ah bh) (lazy-seq (cons ah (merge as bs)))
      (< ah bh) (lazy-seq (cons ah (merge as b)))
      :else (lazy-seq (cons bh (merge a bs))))))

(defn merge-all [stuff]
  (cond
    (< (count stuff) 2) (first stuff)
    (= (count stuff) 2) (merge (first stuff) (second stuff))
    :else
    (let [[a b] (split-at (quot (count stuff) 2) stuff)]
      (merge (merge-all a) (merge-all b)))))


(defn invalid [lo hi nums]
  (->> nums
       (filter #(<= lo %))
       (take-while #((fnil <= hi) % hi))))

(def K {2 [11         101        1001       10001      100001]
        3 [111        10101      1001001    100010001]
        4 [1111       1010101    101010101]
        5 [11111      101010101]
        6 [111111]
        7 [1111111]})


(defn invalid-for-reps
  [k]
  (->> (mapv #(map (fn [x] (* %1 x)) (range %2 %3))
             (K k)
             (iterate #(* 10 %) 1N)
             (iterate #(* 10 %) 10N))
       merge-all))


(defn solve [data rs]
  (loop [res 0
         [[lo hi] & rngs :as rr] data
         [i & is :as invalids] (->> rs (map invalid-for-reps) merge-all)]
    (cond
      (empty? rr)  res
      (< i lo)     (recur res rr is)
      (<= lo i hi) (recur (+ res i) rr is)
      (< hi i)     (recur res rngs invalids))))

(defn part1 [data]
  (solve data [2]))


(defn part2 [data]
  (solve data [2 3 4 5 6 7]))


(when (= *file* (System/getProperty "babashka.file"))
  (let [input (->> *command-line-args* last slurp parse)
        res {:part1 (part1 input)
             :part2 (part2 input)}]
    (prn res)))