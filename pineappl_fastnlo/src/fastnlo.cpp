#include "pineappl_fastnlo/src/fastnlo.hpp"

#include <algorithm>
#include <iterator>
#include <string>

template <typename T>
rust::Vec<T> std_vector_to_rust_vec(std::vector<T> vector)
{
    rust::Vec<T> result;
    result.reserve(vector.size());
    std::move(vector.begin(), vector.end(), std::back_inserter(result));
    return result;
}

rust::Vec<double> CalcPDFLinearCombination(
    fastNLOPDFLinearCombinations const& lc,
    fastNLOCoeffAddBase const& base,
    rust::Slice<double const> pdfx1,
    rust::Slice<double const> pdfx2,
    bool pdf2IsAntiParticle
) {
    std::vector<double> fx1(pdfx1.begin(), pdfx1.end());
    std::vector<double> fx2(pdfx2.begin(), pdfx2.end());

    return std_vector_to_rust_vec(lc.CalcPDFLinearCombination(&base, fx1, fx2, pdf2IsAntiParticle));
}

rust::Vec<double> GetScaleNodes(fastNLOCoeffAddFix const& coeffs, int iObs, int iSvar)
{
    return std_vector_to_rust_vec(coeffs.GetScaleNodes(iObs, iSvar));
}

rust::Vec<double> GetXNodes1(fastNLOCoeffAddBase const& coeffs, int iObsBin)
{
    return std_vector_to_rust_vec(coeffs.GetXNodes1(iObsBin));
}

rust::Vec<double> GetXNodes2(fastNLOCoeffAddBase const& coeffs, int iObsBin)
{
    return std_vector_to_rust_vec(coeffs.GetXNodes2(iObsBin));
}

std::unique_ptr<fastNLOLHAPDF> make_fastnlo_lhapdf_with_name_file_set(
    rust::Str name,
    rust::Str LHAPDFfile,
    int PDFSet
) {
    std::string arg0(name.begin(), name.end());
    std::string arg1(LHAPDFfile.begin(), LHAPDFfile.end());

    return std::unique_ptr<fastNLOLHAPDF>(new fastNLOLHAPDF(arg0, arg1, PDFSet));
}

rust::Vec<double> GetCrossSection(fastNLOReader& reader, bool lNorm)
{
    return std_vector_to_rust_vec(reader.GetCrossSection(lNorm));
}

rust::Vec<double> GetScaleNodes1(fastNLOCoeffAddFlex const& coeffs, int iObsBin)
{
    return std_vector_to_rust_vec(coeffs.GetScaleNodes1(iObsBin));
}

rust::Vec<double> GetScaleNodes2(fastNLOCoeffAddFlex const& coeffs, int iObsBin)
{
    return std_vector_to_rust_vec(coeffs.GetScaleNodes2(iObsBin));
}

std::size_t GetPDFCoeffSize(fastNLOCoeffAddBase const& coeffs)
{
    return coeffs.GetPDFCoeff().size();
}

rust::Vec<pair_int_int> GetPDFCoeff(fastNLOCoeffAddBase const& coeffs, std::size_t index)
{
    auto const& cpp_result = coeffs.GetPDFCoeff().at(index);
    rust::Vec<pair_int_int> result;
    result.reserve(cpp_result.size());

    for (auto const& pair : cpp_result)
    {
        pair_int_int res;
        res.first = pair.first;
        res.second = pair.second;
        result.push_back(res);
    }

    return result;
}

double GetSigmaTilde(
    fastNLOCoeffAddFlex const& coeffs,
    std::size_t mu,
    std::size_t obs,
    std::size_t ix,
    std::size_t is1,
    std::size_t is2,
    int subproc
) {
    return coeffs.GetSigmaTildes().at(mu)->at(obs).at(ix).at(is1).at(is2).at(subproc);
}

std::size_t GetNx(fastNLOCoeffAddFlex const& coeffs, std::size_t obs)
{
    return coeffs.GetSigmaTildes().at(0)->at(obs).size();
}

fastNLOCoeffAddBase const& downcast_coeff_add_fix_to_base(fastNLOCoeffAddFix const& coeffs)
{
    return coeffs;
}

fastNLOCoeffAddBase const& downcast_coeff_add_flex_to_base(fastNLOCoeffAddFlex const& coeffs)
{
    return coeffs;
}

fastNLOReader const& downcast_lhapdf_to_reader(fastNLOLHAPDF const& lhapdf)
{
    return lhapdf;
}

fastNLOReader& downcast_lhapdf_to_reader_mut(fastNLOLHAPDF& lhapdf)
{
    return lhapdf;
}

fastNLOTable const& downcast_lhapdf_to_table(fastNLOLHAPDF const& lhapdf)
{
    return lhapdf;
}

fastNLOCoeffAddFix const* dynamic_cast_coeff_add_fix(fastNLOCoeffBase const* coeffs)
{
    return dynamic_cast <fastNLOCoeffAddFix const*> (coeffs);
}

fastNLOCoeffAddFlex const* dynamic_cast_coeff_add_flex(fastNLOCoeffBase const* coeffs)
{
    return dynamic_cast <fastNLOCoeffAddFlex const*> (coeffs);
}

fastNLOCoeffData const* dynamic_cast_coeff_data(fastNLOCoeffBase const* coeffs)
{
    return dynamic_cast <fastNLOCoeffData const*> (coeffs);
}

fastNLOCoeffMult const* dynamic_cast_coeff_mult(fastNLOCoeffBase const* coeffs)
{
    return dynamic_cast <fastNLOCoeffMult const*> (coeffs);
}

fastNLOPDFLinearCombinations const& downcast_reader_to_pdf_linear_combinations(
    fastNLOReader const& reader
) {
    return reader;
}

pair_double_double GetObsBinDimBounds(
    fastNLOTable const& table,
    unsigned int iObs,
    unsigned int iDim
) {
    pair_double_double result;
    auto const cpp_result = table.GetObsBinDimBounds(iObs, iDim);
    result.first = cpp_result.first;
    result.second = cpp_result.second;
    return result;
}
