use hrafnix_shareable_string::SharedStringTranslationMap;
use std::collections::HashMap;

/// Adds translations for design source resolution and refresh diagnostics.
pub(crate) fn add_design_translation_map(translation_map: &mut SharedStringTranslationMap) {
    translation_map.set_translation_key(
        "design_builtin_not_found",
        HashMap::from([
            (
                "en",
                "Built-in component '%{name}' version %{version} is not registered",
            ),
            ("zh", "内置组件“%{name}”的版本 %{version} 未注册"),
            (
                "de",
                "Integrierte Komponente '%{name}' in Version %{version} ist nicht registriert",
            ),
            (
                "es",
                "El componente integrado '%{name}' versión %{version} no está registrado",
            ),
            (
                "fr",
                "Le composant intégré '%{name}' version %{version} n’est pas enregistré",
            ),
            (
                "ja",
                "組み込みコンポーネント '%{name}' のバージョン %{version} は登録されていません",
            ),
            (
                "ko",
                "내장 컴포넌트 '%{name}'의 버전 %{version}이(가) 등록되어 있지 않습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "design_package_not_registered",
        HashMap::from([
            ("en", "Package '%{name}' version %{version} has no registered root"),
            ("zh", "软件包“%{name}”的版本 %{version} 没有注册的根目录"),
            ("de", "Für Paket '%{name}' in Version %{version} ist kein Stammverzeichnis registriert"),
            ("es", "El paquete '%{name}' versión %{version} no tiene una raíz registrada"),
            ("fr", "Le paquet '%{name}' version %{version} n’a pas de répertoire racine enregistré"),
            ("ja", "パッケージ '%{name}' のバージョン %{version} にはルートディレクトリが登録されていません"),
            ("ko", "패키지 '%{name}'의 버전 %{version}에 등록된 루트 디렉터리가 없습니다"),
        ]),
    );
    translation_map.set_translation_key(
        "design_component_not_found",
        HashMap::from([
            ("en", "Component instance '%{component}' was not found"),
            ("zh", "找不到组件实例“%{component}”"),
            (
                "de",
                "Komponenteninstanz '%{component}' wurde nicht gefunden",
            ),
            (
                "es",
                "No se encontró la instancia del componente '%{component}'",
            ),
            (
                "fr",
                "L’instance du composant '%{component}' est introuvable",
            ),
            (
                "ja",
                "コンポーネントインスタンス '%{component}' が見つかりません",
            ),
            (
                "ko",
                "컴포넌트 인스턴스 '%{component}'을(를) 찾을 수 없습니다",
            ),
        ]),
    );
    translation_map.set_translation_key(
        "design_component_source_mismatch",
        HashMap::from([
            ("en", "Component instance '%{component}' was resolved using different source provenance"),
            ("zh", "组件实例“%{component}”使用了不同的来源信息进行解析"),
            ("de", "Komponenteninstanz '%{component}' wurde mit einer anderen Quellenherkunft aufgelöst"),
            ("es", "La instancia del componente '%{component}' se resolvió con una procedencia de origen diferente"),
            ("fr", "L’instance du composant '%{component}' a été résolue avec une provenance de source différente"),
            ("ja", "コンポーネントインスタンス '%{component}' は異なるソース情報で解決されました"),
            ("ko", "컴포넌트 인스턴스 '%{component}'이(가) 다른 소스 출처로 확인되었습니다"),
        ]),
    );
}
