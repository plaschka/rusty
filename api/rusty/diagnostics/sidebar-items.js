initSidebarItems({"constant":[["INTERNAL_LLVM_ERROR",""]],"enum":[["Diagnostic",""],["ErrNo",""],["Severity","a diagnostics severity"]],"struct":[["AssessedDiagnostic",""],["ClangFormatDiagnosticReporter","a DiagnosticReporter that reports diagnostics using clang-format"],["CodeSpanDiagnosticReporter","a DiagnosticReporter that reports diagnostics using codespan_reporting"],["DefaultDiagnosticAssessor","the default assessor will treat ImprovementSuggestions as warnings and everything else as errors"],["Diagnostician","the Diagnostician handle’s Diangostics with the help of a assessor and a reporter"],["NullDiagnosticReporter","a DiagnosticReporter that swallows all diagnostics"]],"trait":[["DiagnosticAssessor","the assessor determins the severity of a diagnostic this trait allows for different implementations for different usecases (e.g. default, compiler-settings, tests)"],["DiagnosticReporter","the DiagnosticReporter decides on the format and where to report the diagnostic to. possible implementations could print to either std-out, std-err or a file, etc."]]});