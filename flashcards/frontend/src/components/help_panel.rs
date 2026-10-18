use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HelpPanelProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(HelpPanel)]
pub fn help_panel(props: &HelpPanelProps) -> Html {
    html! {
        <div class="help-backdrop">
            <section class="help-modal panel">
                <div class="help-modal-header">
                    <h2 class="panel-title help-modal-title">{"Help & Information"}</h2>
                    <button class="btn btn-secondary btn-small help-close-btn" onclick={props.on_close.clone()}>
                        {"✕ Close"}
                    </button>
                </div>

                <div class="help-body">
                    <div class="help-section">
                        <h3 class="help-section-title">{"About"}</h3>
                        <p class="help-text">
                            {"Language Flashcards is a browser-based vocabulary study tool. \
                            All your data is stored locally in your browser — nothing is sent to a server. \
                            You can organise cards into named datasets, study them in normal or reverse order, \
                            and track which words you have already mastered."}
                        </p>
                        <ul class="help-list">
                            <li>{"Works entirely offline after the page loads."}</li>
                            <li>{"Progress is saved automatically between sessions."}</li>
                            <li>{"Supports Chinese characters, pinyin, and a translation field."}</li>
                            <li>{"Export your cards at any time as CSV or Excel (.xlsx) from the Wordsets panel."}</li>
                        </ul>
                    </div>

                    <hr class="help-divider" />

                    <div class="help-section">
                        <h3 class="help-section-title">{"How to use"}</h3>

                        <div class="help-step">
                            <span class="help-step-number">{"1"}</span>
                            <div>
                                <strong>{"Create or select a wordset"}</strong>
                                <p class="help-text">
                                    {"In Wordsets, choose New wordset, give it a name, then press Create wordset (or Enter). \
                                    The name field is already open for your first collection. \
                                    Choose an existing wordset from Your wordsets to switch to it."}
                                </p>
                            </div>
                        </div>

                        <div class="help-step">
                            <span class="help-step-number">{"2"}</span>
                            <div>
                                <strong>{"Import a CSV or XLSX file"}</strong>
                                <p class="help-text">
                                    {"After creating a new empty wordset, an Import option appears inside the Wordsets panel. Click "}
                                    <em>{"Choose CSV or XLSX"}</em>
                                    {" and select a UTF-8 CSV (.csv) or Excel (.xlsx) file. \
                                    XLSX imports read only the first worksheet, using columns A–E. \
                                    Each row must have three columns, plus optional example sentence and known columns:"}
                                </p>
                                <div class="csv-format-block">
                                    <code>{"word, pinyin, translation, example_sentence, known"}</code>
                                </div>
                                <ul class="help-list">
                                    <li><strong>{"word"}</strong>{" — the term to study (e.g., a Chinese character)."}</li>
                                    <li><strong>{"pinyin"}</strong>{" — pronunciation hint, optional."}</li>
                                    <li><strong>{"translation"}</strong>{" — the meaning in your language."}</li>
                                    <li><strong>{"example_sentence"}</strong>{" — optional example sentence in column 4. Click or tap Example sentence below a flashcard to reveal it."}</li>
                                    <li><strong>{"known"}</strong>{" — optional column 5: write "}<code>{"true"}</code>{" if already mastered, otherwise leave blank or write "}<code>{"false"}</code>{"."}</li>
                                </ul>
                                <p class="help-text">{"Example row:"}</p>
                                <div class="csv-format-block">
                                    <code>{"你好,nǐ hǎo,Hello,你好，很高兴认识你！,false"}</code>
                                </div>
                                <p class="help-text help-text-muted">
                                    {"Pinyin may be blank, but keep its column between word and translation. \
                                    Word and translation are required. A matching header row is optional. \
                                    Older four-column files with true/false in column 4 still import known status. Use an example_sentence header if your example is literally true or false. \
                                    Invalid encoding, garbled characters, unsupported file types, and invalid rows show an error; no cards are imported until the whole file is valid. \
                                    Import is only available while the wordset is empty, \
                                    so it cannot overwrite cards you have already added."}
                                </p>
                            </div>
                        </div>

                        <div class="help-step">
                            <span class="help-step-number">{"3"}</span>
                            <div>
                                <strong>{"Add cards manually"}</strong>
                                <p class="help-text">
                                    {"Click "}
                                    <em>{"Add your first card"}</em>
                                    {" in an empty wordset, or Add a card in your current wordset, to enter a word and an optional example sentence. Add New Flashcard is also available in Study Controls. Example sentences start hidden on every card; click or tap the Example sentence row to show or hide one."}
                                </p>
                            </div>
                        </div>

                        <div class="help-step">
                            <span class="help-step-number">{"4"}</span>
                            <div>
                                <strong>{"Study"}</strong>
                                <p class="help-text">
                                    {"Click the flashcard to reveal the next stage: \
                                    character → pinyin → translation (or reversed). \
                                    Mark a card as "}
                                    <em>{"Known"}</em>
                                    {" to move it to the Word Review table, \
                                    or use "}
                                    <em>{"Randomize"}</em>
                                    {" to shuffle the order. You can also switch to "}
                                    <em>{"Exercise Mode"}</em>
                                    {" from Study Controls to type an answer instead of flipping stages. \
                                    After pressing Enter or "}
                                    <em>{"Check"}</em>
                                    {", the card highlights green or red, shows the correct answer if needed, and reveals pinyin when available."}
                                </p>
                            </div>
                        </div>

                        <div class="help-step">
                            <span class="help-step-number">{"5"}</span>
                            <div>
                                <strong>{"Word Review table"}</strong>
                                <p class="help-text">
                                    {"The Word Review section lists all words you have marked as known. \
                                    Use "}
                                    <em>{"Restore"}</em>
                                    {" to move a card back to the Flashcards section for further practice, \
                                    or "}
                                    <em>{"Delete"}</em>
                                    {" to remove it permanently."}
                                </p>
                                <p class="help-text">
                                    {"Click "}
                                    <em>{"Show Unknown Words"}</em>
                                    {" to also show cards still in the Flashcards section. \
                                    Unknown cards appear with an \"Unknown\" badge and let you \
                                    mark them as known or delete them directly from the table — \
                                    useful when you want to review all words in one place without \
                                    navigating card by card."}
                                </p>
                            </div>
                        </div>

                        <div class="help-step">
                            <span class="help-step-number">{"6"}</span>
                            <div>
                                <strong>{"Rename a wordset"}</strong>
                                <p class="help-text">
                                    {"Select the wordset, open Options, then choose "}
                                    <em>{"Rename wordset"}</em>
                                    {" to edit its name. \
                                    Confirm with "}
                                    <em>{"Save name"}</em>
                                    {" or discard with "}
                                    <em>{"Cancel"}</em>
                                    {". Renaming is blocked if the new name is empty or already taken."}
                                </p>
                            </div>
                        </div>

                        <div class="help-step">
                            <span class="help-step-number">{"7"}</span>
                            <div>
                                <strong>{"Export the active wordset"}</strong>
                                <p class="help-text">
                                    {"Open Export wordset, then choose "}
                                    <em>{"Export CSV"}</em>
                                    {" or "}
                                    <em>{"Export XLSX"}</em>
                                    {" in the Wordsets panel to save the currently selected wordset's cards \
                                    (including example sentences and known/unknown status) for backup or sharing. \
                                    Both formats include headers and can be imported into an empty wordset. \
                                    Export is available whenever a wordset is selected."}
                                </p>
                            </div>
                        </div>

                        <div class="help-step">
                            <span class="help-step-number">{"8"}</span>
                            <div>
                                <strong>{"Save and restore your entire progress"}</strong>
                                <p class="help-text">
                                    {"The "}
                                    <em>{"Backup & restore"}</em>
                                    {" section at the bottom of Wordsets lets you download and restore your complete app state:"}
                                </p>
                                <ul class="help-list">
                                    <li><strong>{"Download full backup"}</strong>{" — downloads every wordset and your current study position (which card you're on, normal vs. reverse mode, etc.) as a single JSON backup file."}</li>
                                    <li><strong>{"Restore a backup"}</strong>{" — restores your entire app state from a previously saved backup file. This replaces all current wordsets and study progress."}</li>
                                </ul>
                                <p class="help-text help-text-muted">
                                    {"Use this feature to switch devices, back up your learning history, or recover after clearing browser data. \
                                    Unlike Export (which saves only the active wordset as CSV or XLSX), a full backup captures everything in a portable format."}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
