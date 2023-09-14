use solana_security_txt::security_txt;

// Source code is an optional field, contacts is a comma-separated string
// Policy is mandatory?
security_txt! {
    name: "Ephemerality",
    project_url: "",
    contacts: "https://www.twitter.com/burger606",
    policy: "",
    preferred_languages: "en",
    auditors: "TBA",
    acknowledgements: ""
}
