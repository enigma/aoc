#!/usr/bin/env bb

(ns y2025.d08
  (:require [clojure.string :as str]))

(defn parse [s]
  (->> (str/split-lines s)
       (mapv #(mapv parse-long (str/split % #",")))))

(defn find-set [p forest]
  (if-let [[parent size] (forest p)]
    (if (= parent p)
      [parent forest]
      (let [[parent' _size'] (find-set parent forest)]
        [parent' (assoc forest p [parent' size])]))
    [p (assoc forest p [p 1])]))


(defn top3sizes [forest]
  (loop [[k & ks] (keys forest)
         f forest
         roots #{}]
    (if (nil? k)
      (->> roots (sort-by #(-> f (get %) last) >) (take 3) (map f) (map second))
      (let [[u f] (find-set k f)]
        (if (= u k)
          (recur ks f (conj roots k))
          (recur ks f roots))))))


(defn union-sets [x y forest]
  (let [[x forest] (find-set x forest)
        [y forest] (find-set y forest)
        [x y] (if (< (-> (forest x) last) (-> (forest y) last)) [y x] [x y])]
    (-> forest
        (assoc-in [y 0] x)
        (update-in [x 1] + (-> (forest y) last)))))


(defn dist [p1 p2]
  (apply + (map (fn [a b] (let [d (- a b)] (* d d))) p1 p2)))

(defn solve [pos]
  (let [pairs (sort (for [[i p1] (map-indexed vector pos)
                          p2 (drop (inc i) pos)]
                      [(dist p1 p2) p1 p2]))]
    (loop [n 0
           [[_ p1 p2] & ps] pairs
           forest {}
           part1 nil
           part2 nil]
      (let [part1 (if (= n 1000) (apply * (top3sizes forest)) part1)
            [u forest] (find-set p1 forest)
            [v forest] (find-set p2 forest)]
        (if (not= u v)
          (let [[u forest] (->> forest (union-sets u v) (find-set u))]
            (if (= (count pos) (-> forest (get u) last))
              {:part1 part1 :part2 (* (first p1) (first p2))}
              (recur (inc n) ps forest part1 part2)))
          (recur (inc n) ps forest part1 part2))))))

(when (= *file* (System/getProperty "babashka.file"))
  (let [input (->> *command-line-args* last slurp parse)
        res (solve input)]
    (prn res)))

