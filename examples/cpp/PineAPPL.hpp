/**
 * @file PineAPPL.hpp
 * @brief object oriented interface to PineAPPL
 */
#ifndef PineAPPL_HPP_
#define PineAPPL_HPP_

#include <string>
#include <cstdint>
#include <LHAPDF/LHAPDF.h>

#include <pineappl_capi.h>

/** @brief object oriented interface to PineAPPL  */
namespace PineAPPL {

/** @brief Key-value storage for passing optional information during grid creation */
struct KeyVal {
    /** @brief underlying raw object */ 
    pineappl_keyval *raw;

    /** @brief constructor */
    KeyVal() { this->raw = pineappl_keyval_new(); }

    /** @brief destructor */
    ~KeyVal() { pineappl_keyval_delete(this->raw); }

    
    /** @name setter */
    ///@{
    void set_double(const std::string& key, const double value) const { pineappl_keyval_set_double(this->raw, key.c_str(), value); }
    void set_bool(const std::string& key, const bool value) const { pineappl_keyval_set_bool(this->raw, key.c_str(), value); }
    void set_int(const std::string& key, const int value) const { pineappl_keyval_set_int(this->raw, key.c_str(), value); }
    void set_string(const std::string& key, const std::string& value) const { pineappl_keyval_set_string(this->raw, key.c_str(), value.c_str()); }
    ///@}

    /** @name getter */
    ///@{
    double get_double(const std::string& key) const { return pineappl_keyval_double(this->raw, key.c_str()); }
    bool get_bool(const std::string& key) const { return pineappl_keyval_bool(this->raw, key.c_str()); }
    int get_int(const std::string& key) const { return pineappl_keyval_int(this->raw, key.c_str()); }
    std::string get_string(const std::string& key) const { return pineappl_keyval_string(this->raw, key.c_str()); }
    ///@}

};

/** @brief Entry in luminosity function  */
struct LumiEntry {
    /** @brief first parton id */
    std::int32_t pid1;

    /** @brief second parton id */
    std::int32_t pid2;

    /** @brief relative weight */
    double weight;
};

/** @brief Luminosity function */
struct Lumi {
    /** @brief underlying raw object */
    pineappl_lumi *raw;

    /** @brief constructor */
    Lumi(){ this->raw = pineappl_lumi_new(); }

    /** @brief destructor */
    ~Lumi(){ pineappl_lumi_delete(this->raw); }

    /** @brief number of elements */
    std::size_t count() const { return pineappl_lumi_count(this->raw); }

    /**
     * @brief add a luminosity function
     * @param c luminosity function
     */
    void add(const std::vector<LumiEntry>& c) const {
        const std::size_t n = c.size();
        std::vector<std::int32_t> pids (2*n);
        std::vector<double> weights (n);
        for (std::size_t j = 0; j < n; ++j) {
            pids[2*j] = c[j].pid1;
            pids[2*j+1] = c[j].pid2;
            weights[j] = c[j].weight;
        }
        pineappl_lumi_add(this->raw, n, pids.data(), weights.data());
    }

    /**
     * @brief Returns the number of combinations of the luminosity function `lumi` for the specified entry.
     * @param entry position in lumi
     */
    std::size_t combinations(const std::size_t entry) const { return pineappl_lumi_combinations(this->raw, entry); }
};

/** @brief Coupling powers for each grid. */
struct Order {
    /** @brief Exponent of the strong coupling. */
    uint32_t alphas;

    /** @brief Exponent of the electromagnetic coupling. */
    uint32_t alpha;

    /** @brief Exponent of the logarithm of the scale factor of the renomalization scale. */
    uint32_t logxir;

    /** @brief Exponent of the logarithm of the scale factor of the factorization scale. */
    uint32_t logxif;
};

struct Grid {

    /** @brief underlying raw object */
    pineappl_grid *raw;

    /**
     * @brief constructor
     * @param lumi luminosity
     * @param orders orders
     * @param bin_limits bin limits
     * @param key_val additional informations
     */
    Grid(const Lumi& lumi, const std::vector<Order>& orders, const std::vector<double>& bin_limits, const KeyVal& key_val) {
        // cast orders
        const std::size_t n_orders = orders.size();
        std::vector<std::uint32_t> raw_orders (4 * n_orders);
        for (size_t j = 0; j < n_orders; ++j) {
            raw_orders[4*j + 0] = orders[j].alphas;
            raw_orders[4*j + 1] = orders[j].alpha;
            raw_orders[4*j + 2] = orders[j].logxir;
            raw_orders[4*j + 3] = orders[j].logxif;
        }
        this->raw = pineappl_grid_new(lumi.raw, n_orders, raw_orders.data(), bin_limits.size() - 1, bin_limits.data(), key_val.raw);
    }

    /** @brief destructor */
    ~Grid(){ pineappl_grid_delete(this->raw); }

    /**
     * @brief Number of orders
     * @return number of orders
     */
    std::size_t order_count() const {
        return pineappl_grid_order_count(this->raw);
    }

    /**
     * @brief Number of bins
     * @return number of bins
     */
    std::size_t bin_count() const {
        return pineappl_grid_bin_count(this->raw);
    }

    /**
     * @brief Fill grid for the given parameters
     * @param x1 first momentum fraction
     * @param x2 second momentum fraction
     * @param q2 scale
     * @param order order index
     * @param observable observable value
     * @param lumi luminosity index
     * @param weight weight
     */
    void fill(const double x1, const double x2, const double q2, const std::size_t order, const double observable, const std::size_t lumi, const double weight) const {
        pineappl_grid_fill(this->raw, x1, x2, q2, order, observable, lumi, weight);
    }

    /**
     * @brief perform a convolution of the grid with PDFs
     * @param pdg_id particle ID
     * @param pdf PDF
     * @param xi_ren renormalization scale variation
     * @param xi_fac factorization scale variation
     * @return prediction for each bin
     * @return prediction for each bin
     */
    std::vector<double> convolute_with_one(const std::int32_t pdg_id, LHAPDF::PDF *pdf, const double xi_ren = 1.0, const double xi_fac = 1.0 ) const {
        std::vector<bool> order_mask(this->order_count(), true);
        pineappl_lumi* l = pineappl_grid_lumi(this->raw);
        std::vector<bool> lumi_mask(pineappl_lumi_count(l), true);
        pineappl_lumi_delete(l);
        return this->convolute_with_one(pdg_id, pdf, order_mask, lumi_mask, xi_ren, xi_fac);
    }

    /**
     * @brief perform a convolution of the grid with PDFs
     * @param pdg_id particle ID
     * @param pdf PDF
     * @param order_mask order mask
     * @param lumi_mask luminosity mask
     * @param xi_ren renormalization scale variation
     * @param xi_fac factorization scale variation
     * @return prediction for each bin
     */
    std::vector<double> convolute_with_one(const std::int32_t pdg_id, LHAPDF::PDF *pdf, const std::vector<bool>& order_mask, const std::vector<bool>& lumi_mask, const double xi_ren = 1.0, const double xi_fac = 1.0) const {
        // prepare LHAPDF stuff
        auto xfx = [](std::int32_t id, double x, double q2, void* pdf) {
            return static_cast <LHAPDF::PDF*> (pdf)->xfxQ2(id, x, q2);
        };
        auto alphas = [](double q2, void* pdf) {
            return static_cast <LHAPDF::PDF*> (pdf)->alphasQ2(q2);
        };
        // cast order_mask
        bool *raw_order_mask = new bool[this->order_count()];
        {std::size_t j = 0;
        for (auto it = order_mask.cbegin(); it != order_mask.cend(); ++it, ++j)
            raw_order_mask[j] = *it;}
        pineappl_lumi* l = pineappl_grid_lumi(this->raw);
        const std::size_t n_lumis = pineappl_lumi_count(l);
        pineappl_lumi_delete(l);
        bool *raw_lumi_mask = new bool[n_lumis];
        {std::size_t j = 0;
        for (auto it = lumi_mask.cbegin(); it != lumi_mask.cend(); ++it, ++j)
            raw_lumi_mask[j] = *it;}
        std::vector<double> results(this->bin_count());
        pineappl_grid_convolute_with_one(this->raw, pdg_id, xfx, alphas, pdf, raw_order_mask, raw_lumi_mask, xi_ren, xi_fac, results.data());
        delete[] raw_order_mask;
        delete[] raw_lumi_mask;
        return results;
    }

    /**
     * @brief Write grid to file
     * @param filename file name
     */
    void write(const std::string &filename) const {
        pineappl_grid_write(this->raw, filename.c_str());
    }

    /**
     * @brief Set a metadata entry
     * @param key key
     * @param value value
     */
    void set_key_value(const std::string &key, const std::string &value) const {
        pineappl_grid_set_key_value(this->raw, key.c_str(), value.c_str());
    }

    /**
     * @brief Get a metadata entry
     * @param key key
     * @return value
     */
    std::string get_key_value(const std::string &key) const {
        auto* value = pineappl_grid_key_value(this->raw, key.c_str());
        const std::string res(value);
        // delete the allocated object
        pineappl_string_delete(value);
        return res;
    }
};

}


#endif // PineAPPL_HPP_
