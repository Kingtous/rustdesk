lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Κατάσταση"),
        ("Your Desktop", "Ο σταθμός εργασίας σας"),
        ("desk_tip", "Η πρόσβαση στον σταθμό εργασίας σας είναι δυνατή με αυτό το αναγνωριστικό και τον κωδικό πρόσβασης."),
        ("Password", "Κωδικός πρόσβασης"),
        ("Ready", "Έτοιμο"),
        ("Established", "Συνδέθηκε"),
        ("connecting_status", "Σύνδεση στο δίκτυο RustDesk..."),
        ("Enable Service", "Ενεργοποίηση υπηρεσίας"),
        ("Start Service", "Έναρξη υπηρεσίας"),
        ("Service is running", "Η υπηρεσία εκτελείται"),
        ("Service is not running", "Η υπηρεσία δεν εκτελείται"),
        ("not_ready_status", "Δεν είναι έτοιμο. Ελέγξτε τη σύνδεσή σας"),
        ("Control Remote Desktop", "Έλεγχος απομακρυσμένου σταθμού εργασίας"),
        ("Transfer File", "Μεταφορά αρχείου"),
        ("Connect", "Σύνδεση"),
        ("Recent Sessions", "Πρόσφατες συνεδρίες"),
        ("Address Book", "Βιβλίο διευθύνσεων"),
        ("Confirmation", "Επιβεβαίωση"),
        ("TCP Tunneling", "TCP Tunneling"),
        ("Remove", "Κατάργηση"),
        ("Refresh random password", "Νέος τυχαίος κωδικός πρόσβασης"),
        ("Set your own password", "Ορίστε τον δικό σας κωδικό πρόσβασης"),
        ("Enable Keyboard/Mouse", "Ενεργοποίηση πληκτρολογίου/ποντικιού"),
        ("Enable Clipboard", "Ενεργοποίηση Προχείρου"),
        ("Enable File Transfer", "Ενεργοποίηση μεταφοράς αρχείων"),
        ("Enable TCP Tunneling", "Ενεργοποίηση TCP Tunneling"),
        ("IP Whitelisting", "Λίστα επιτρεπόμενων IP"),
        ("ID/Relay Server", "Διακομιστής ID/Αναμετάδοσης"),
        ("Import Server Config", "Εισαγωγή διαμόρφωσης διακομιστή"),
        ("Export Server Config", "Εξαγωγή διαμόρφωσης διακομιστή"),
        ("Import server configuration successfully", "Επιτυχής εισαγωγή διαμόρφωσης διακομιστή"),
        ("Export server configuration successfully", "Επιτυχής εξαγωγή διαμόρφωσης διακομιστή"),
        ("Invalid server configuration", "Μη έγκυρη διαμόρφωση διακομιστή"),
        ("Clipboard is empty", "Το πρόχειρο είναι κενό"),
        ("Stop service", "Διακοπή υπηρεσίας"),
        ("Change ID", "Αλλαγή αναγνωριστικού ID"),
        ("Website", "Ιστότοπος"),
        ("About", "Πληροφορίες"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Σίγαση"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Είσοδος ήχου"),
        ("Enhancements", "Βελτιώσεις"),
        ("Hardware Codec", "Κωδικοποιητής υλικού"),
        ("Adaptive Bitrate", "Adaptive Bitrate"),
        ("ID Server", "Διακομιστής ID"),
        ("Relay Server", "Διακομιστής αναμετάδοσης"),
        ("API Server", "Διακομιστής API"),
        ("invalid_http", "Πρέπει να ξεκινά με http:// ή https://"),
        ("Invalid IP", "Μη έγκυρη διεύθυνση IP"),
        ("id_change_tip", "Επιτρέπονται μόνο οι χαρακτήρες a-z, A-Z, 0-9 και _ (υπογράμμιση). Το πρώτο γράμμα πρέπει να είναι a-z, A-Z και το μήκος πρέπει να είναι μεταξύ 6 και 16 χαρακτήρων."),
        ("Invalid format", "Μη έγκυρη μορφή"),
        ("server_not_support", "Αυτή η δυνατότητα δεν υποστηρίζεται ακόμη από τον διακομιστή"),
        ("Not available", "Μη διαθέσιμο"),
        ("Too frequent", "Πολύ συχνά"),
        ("Cancel", "Ακύρωση"),
        ("Skip", "Παράλειψη"),
        ("Close", "Κλείσιμο"),
        ("Retry", "Δοκίμασε ξανά"),
        ("OK", "Εντάξει"),
        ("Password Required", "Απαιτείται κωδικός πρόσβασης"),
        ("Please enter your password", "Παρακαλώ εισάγετε τον κωδικό πρόσβασης"),
        ("Remember password", "Απομνημόνευση κωδικού πρόσβασης"),
        ("Wrong Password", "Λάθος κωδικός πρόσβασης"),
        ("Do you want to enter again?", "Επανασύνδεση;"),
        ("Connection Error", "Σφάλμα σύνδεσης"),
        ("Error", "Σφάλμα"),
        ("Reset by the peer", "Η σύνδεση επαναφέρθηκε από τον απομακρυσμένο σταθμό"),
        ("Connecting...", "Σύνδεση..."),
        ("Connection in progress. Please wait.", "Σύνδεση σε εξέλιξη. Παρακαλώ περιμένετε."),
        ("Please try 1 minute later", "Παρακαλώ ξαναδοκιμάστε σε 1 λεπτό"),
        ("Login Error", "Σφάλμα εισόδου"),
        ("Successful", "Επιτυχής"),
        ("Connected, waiting for image...", "Συνδέθηκε, αναμονή για εικόνα..."),
        ("Name", "Όνομα"),
        ("Type", "Τύπος"),
        ("Modified", "Τροποποιήθηκε"),
        ("Size", "Μέγεθος"),
        ("Show Hidden Files", "Εμφάνιση κρυφών αρχείων"),
        ("Receive", "Λήψη"),
        ("Send", "Αποστολή"),
        ("Refresh File", "Ανανέωση αρχείου"),
        ("Local", "Τοπικό"),
        ("Remote", "Απομακρυσμένο"),
        ("Remote Computer", "Απομακρυσμένος υπολογιστής"),
        ("Local Computer", "Τοπικός υπολογιστής"),
        ("Confirm Delete", "Επιβεβαίωση διαγραφής"),
        ("Delete", "Διαγραφή"),
        ("Properties", "Ιδιότητες"),
        ("Multi Select", "Πολλαπλή επιλογή"),
        ("Select All", "Επιλογή όλων"),
        ("Unselect All", "Κατάργηση επιλογής όλων"),
        ("Empty Directory", "Κενός φάκελος"),
        ("Not an empty directory", "Ο φάκελος δεν είναι κενός"),
        ("Are you sure you want to delete this file?", "Είστε βέβαιοι ότι θέλετε να διαγράψετε αυτό το αρχείο;"),
        ("Are you sure you want to delete this empty directory?", "Είστε βέβαιοι ότι θέλετε να διαγράψετε αυτόν τον κενό φάκελο;"),
        ("Are you sure you want to delete the file of this directory?", "Είστε βέβαιοι ότι θέλετε να διαγράψετε το αρχείο αυτού του φακέλου;"),
        ("Do this for all conflicts", "Κάνε αυτό για όλες τις διενέξεις"),
        ("This is irreversible!", "Αυτό είναι μη αναστρέψιμο!"),
        ("Deleting", "Διαγραφή"),
        ("files", "αρχεία"),
        ("Waiting", "Αναμονή"),
        ("Finished", "Ολοκληρώθηκε"),
        ("Speed", "Ταχύτητα"),
        ("Custom Image Quality", "Προσαρμοσμένη ποιότητα εικόνας"),
        ("Privacy mode", "Λειτουργία απορρήτου"),
        ("Block user input", "Αποκλεισμός χειρισμού από τον χρήστη"),
        ("Unblock user input", "Κατάργηση αποκλεισμού χειρισμού από τον χρήστη"),
        ("Adjust Window", "Προσαρμογή παραθύρου"),
        ("Original", "Πρωτότυπο"),
        ("Shrink", "Συρρίκνωση"),
        ("Stretch", "Προσαρμογή"),
        ("Scrollbar", "Γραμμή κύλισης"),
        ("ScrollAuto", "Αυτόματη κύλιση"),
        ("Good image quality", "Καλή ποιότητα εικόνας"),
        ("Balanced", "Ισορροπημένο"),
        ("Optimize reaction time", "Βελτιστοποίηση χρόνου αντίδρασης"),
        ("Custom", "Προσαρμογή ποιότητας εικόνας"),
        ("Show remote cursor", "Εμφάνιση απομακρυσμένου κέρσορα"),
        ("Show quality monitor", "Εμφάνιση παρακολούθησης ποιότητας σύνδεσης"),
        ("Disable clipboard", "Απενεργοποίηση προχείρου"),
        ("Lock after session end", "Κλείδωμα μετά το τέλος της συνεδρίας"),
        ("Insert", ""),
        ("Insert Lock", "Κλείδωμα απομακρυσμένου σταθμού"),
        ("Refresh", "Ανανέωση"),
        ("ID does not exist", "Το αναγνωριστικό ID δεν υπάρχει"),
        ("Failed to connect to rendezvous server", "Αποτυχία σύνδεσης με διακομιστή"),
        ("Please try later", "Παρακαλώ δοκιμάστε αργότερα"),
        ("Remote desktop is offline", "Ο απομακρυσμένος σταθμός εργασίας είναι εκτός σύνδεσης"),
        ("Key mismatch", "Μη έγκυρο κλειδί"),
        ("Timeout", "Τέλος χρόνου"),
        ("Failed to connect to relay server", "Αποτυχία σύνδεσης με διακομιστή αναμετάδοσης"),
        ("Failed to connect via rendezvous server", "Απέτυχε η σύνδεση μέσω διακομιστή"),
        ("Failed to connect via relay server", "Απέτυχε η σύνδεση μέσω διακομιστή αναμετάδοσης"),
        ("Failed to make direct connection to remote desktop", "Απέτυχε η απευθείας σύνδεση με τον απομακρυσμένο σταθμό εργασίας"),
        ("Set Password", "Ορίστε κωδικό"),
        ("OS Password", "Κωδικός πρόσβασης λειτουργικού συστήματος"),
        ("install_tip", "Λόγω UAC, το RustDesk ενδέχεται να μην λειτουργεί σωστά σε ορισμένες περιπτώσεις. Για να αποφύγετε το UAC, κάντε κλικ στο κουμπί παρακάτω για να εγκαταστήσετε το RustDesk στο σύστημα"),
        ("Click to upgrade", "Κάντε κλικ για αναβάθμιση"),
        ("Click to download", "Κάντε κλικ για λήψη"),
        ("Click to update", "Κάντε κλικ για ενημέρωση"),
        ("Configure", "Διαμόρφωση"),
        ("config_acc", "Για τον απομακρυσμένο έλεγχο του υπολογιστή σας, πρέπει να εκχωρήσετε δικαιώματα πρόσβασης στο RustDesk."),
        ("config_screen", "Για να αποκτήσετε απομακρυσμένη πρόσβαση στον υπολογιστή σας, πρέπει να εκχωρήσετε το δικαίωμα RustDesk \"Screen Capture\"."),
        ("Installing ...", "Εγκατάσταση ..."),
        ("Install", "Εγκατάσταση"),
        ("Installation", "Εγκατάσταση"),
        ("Installation Path", "Διαδρομή εγκατάστασης"),
        ("Create start menu shortcuts", "Δημιουργία συντομεύσεων μενού έναρξης"),
        ("Create desktop icon", "Δημιουργία εικονιδίου επιφάνειας εργασίας"),
        ("agreement_tip", "Με την εγκατάσταση αποδέχεστε την άδεια χρήσης"),
        ("Accept and Install", "Αποδοχή και εγκατάσταση"),
        ("End-user license agreement", "Σύμβαση άδειας χρήσης τελικού χρήστη"),
        ("Generating ...", "Δημιουργία ..."),
        ("Your installation is lower version.", "Η έκδοση της εγκατάστασής σας είναι παλαιότερη."),
        ("not_close_tcp_tip", "Μην κλείσετε αυτό το παράθυρο ενώ χρησιμοποιείτε το τούνελ."),
        ("Listening ...", "Αναμονή ..."),
        ("Remote Host", "Απομακρυσμένος υπολογιστής"),
        ("Remote Port", "Απομακρυσμένη θύρα"),
        ("Action", "Δράση"),
        ("Add", "Προσθήκη"),
        ("Local Port", "Τοπική θύρα"),
        ("Local Address", "Τοπική διεύθυνση"),
        ("Change Local Port", "Αλλαγή τοπικής θύρας"),
        ("setup_server_tip", "Για πιο γρήγορη σύνδεση, ρυθμίστε τον δικό σας διακομιστή σύνδεσης"),
        ("Too short, at least 6 characters.", "Πολύ μικρό, τουλάχιστον 6 χαρακτήρες."),
        ("The confirmation is not identical.", "Η επιβεβαίωση δεν είναι πανομοιότυπη."),
        ("Permissions", "Άδειες"),
        ("Accept", "Αποδοχή"),
        ("Dismiss", "Απόρριψη"),
        ("Disconnect", "Αποσύνδεση"),
        ("Allow using keyboard and mouse", "Να επιτρέπεται η χρήση πληκτρολογίου και ποντικιού"),
        ("Allow using clipboard", "Να επιτρέπεται η χρήση του προχείρου"),
        ("Allow hearing sound", "Να επιτρέπεται η αναπαραγωγή ήχου"),
        ("Allow file copy and paste", "Να επιτρέπεται η αντιγραφή και επικόλληση αρχείου"),
        ("Connected", "Συνδεδεμένο"),
        ("Direct and encrypted connection", "Άμεση και κρυπτογραφημένη σύνδεση"),
        ("Relayed and encrypted connection", "Κρυπτογραφημένη σύνδεση με αναμετάδοση"),
        ("Direct and unencrypted connection", "Άμεση και μη κρυπτογραφημένη σύνδεση"),
        ("Relayed and unencrypted connection", "Μη κρυπτογραφημένη σύνδεση με αναμετάδοση"),
        ("Enter Remote ID", "Εισαγωγή απομακρυσμένου ID"),
        ("Enter your password", "Εισάγετε τον κωδικό σας"),
        ("Logging in...", "Σύνδεση..."),
        ("Enable RDP session sharing", "Ενεργοποίηση κοινής χρήσης RDP"),
        ("Auto Login", "Αυτόματη είσοδος"),
        ("Enable Direct IP Access", "Ενεργοποίηση άμεσης πρόσβασης IP"),
        ("Rename", "Μετονομασία"),
        ("Space", "Χώρος"),
        ("Create Desktop Shortcut", "Δημιουργία συντόμευσης στην επιφάνεια εργασίας"),
        ("Change Path", "Αλλαγή διαδρομής"),
        ("Create Folder", "Δημιουργία φακέλου"),
        ("Please enter the folder name", "Παρακαλώ εισάγετε το όνομα του φακέλου"),
        ("Fix it", "Επιδιόρθωσε το"),
        ("Warning", "Προειδοποίηση"),
        ("Login screen using Wayland is not supported", "Η οθόνη εισόδου με χρήση του Wayland δεν υποστηρίζεται"),
        ("Reboot required", "Απαιτείται επανεκκίνηση"),
        ("Unsupported display server ", "Μη υποστηριζόμενος διακομιστής εμφάνισης "),
        ("x11 expected", "απαιτείται X11"),
        ("Port", "Θύρα"),
        ("Settings", "Ρυθμίσεις"),
        ("Username", "Όνομα χρήστη"),
        ("Invalid port", "Μη έγκυρη θύρα"),
        ("Closed manually by the peer", "Έκλεισε από τον απομακρυσμένο σταθμό"),
        ("Enable remote configuration modification", "Ενεργοποίηση απομακρυσμένης τροποποίησης ρυθμίσεων"),
        ("Run without install", "Εκτέλεση χωρίς εγκατάσταση"),
        ("Always connected via relay", "Πάντα συνδεδεμένο μέσω αναμετάδοσης"),
        ("Always connect via relay", "Σύνδεση πάντα μέσω αναμετάδοσης"),
        ("whitelist_tip", "Μόνο οι IP της λίστας επιτρεπόμενων έχουν πρόσβαση"),
        ("Login", "Σύνδεση"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Αποσύνδεση"),
        ("Tags", "Ετικέτες"),
        ("Search ID", "Αναζήτηση ID"),
        ("Current Wayland display server is not supported", "Ο τρέχων διακομιστής εμφάνισης Wayland δεν υποστηρίζεται"),
        ("whitelist_sep", "Διαχωρίζονται με κόμμα, ερωτηματικό, διάστημα ή νέα γραμμή"),
        ("Add ID", "Προσθήκη αναγνωριστικού ID"),
        ("Add Tag", "Προσθήκη ετικέτας"),
        ("Unselect all tags", "Κατάργηση επιλογής όλων των ετικετών"),
        ("Network error", "Σφάλμα δικτύου"),
        ("Username missed", "Δεν συμπληρώσατε το όνομα χρήστη"),
        ("Password missed", "Δεν συμπληρώσατε τον κωδικό πρόσβασης"),
        ("Wrong credentials", "Λάθος διαπιστευτήρια"),
        ("Edit Tag", "Επεξεργασία ετικέτας"),
        ("Unremember Password", "Διαγραφή απομνημονευμένου κωδικού"),
        ("Favorites", "Αγαπημένα"),
        ("Add to Favorites", "Προσθήκη στα αγαπημένα"),
        ("Remove from Favorites", "Κατάργηση από τα Αγαπημένα"),
        ("Empty", "Άδειο"),
        ("Invalid folder name", "Μη έγκυρο όνομα φακέλου"),
        ("Socks5 Proxy", "Διαμεσολαβητής Socks5"),
        ("Hostname", "Όνομα υπολογιστή"),
        ("Discovered", "Ανακαλύφθηκε"),
        ("install_daemon_tip", "Για να ξεκινά με την εκκίνηση του υπολογιστή, πρέπει να εγκαταστήσετε την υπηρεσία συστήματος"),
        ("Remote ID", "Απομακρυσμένο ID"),
        ("Paste", "Επικόλληση"),
        ("Paste here?", "Επικόλληση εδώ;"),
        ("Are you sure to close the connection?", "Είστε βέβαιοι ότι θέλετε να κλείσετε αυτήν τη σύνδεση;"),
        ("Download new version", "Λήψη νέας έκδοσης"),
        ("Touch mode", "Λειτουργία αφής"),
        ("Mouse mode", "Λειτουργία ποντικιού"),
        ("One-Finger Tap", "Πάτημα με ένα δάχτυλο"),
        ("Left Mouse", "Αριστερό κλικ"),
        ("One-Long Tap", "Παρατεταμένο πάτημα με ένα δάχτυλο"),
        ("Two-Finger Tap", "Πάτημα με δύο δάχτυλα"),
        ("Right Mouse", "Δεξί κλικ"),
        ("One-Finger Move", "Κίνηση με ένα δάχτυλο"),
        ("Double Tap & Move", "Διπλό πάτημα και μετακίνηση"),
        ("Mouse Drag", "Σύρετε το ποντίκι"),
        ("Three-Finger vertically", "Τρία δάχτυλα, κάθετα"),
        ("Mouse Wheel", "Τροχός ποντικιού"),
        ("Two-Finger Move", "Κίνηση με δύο δάχτυλα"),
        ("Canvas Move", "Κίνηση καμβά"),
        ("Pinch to Zoom", "Τσίμπημα για ζουμ"),
        ("Canvas Zoom", "Ζουμ σε καμβά"),
        ("Reset canvas", "Επαναφορά καμβά"),
        ("No permission of file transfer", "Δεν υπάρχει άδεια για μεταφορά αρχείων"),
        ("Note", "Σημείωση"),
        ("Connection", "Σύνδεση"),
        ("Share Screen", "Κοινή χρήση οθόνης"),
        ("CLOSE", "Απενεργοποίηση"),
        ("OPEN", "Ενεργοποίηση"),
        ("Chat", "Κουβέντα"),
        ("Total", "Σύνολο"),
        ("items", "στοιχεία"),
        ("Selected", "Επιλεγμένο"),
        ("Screen Capture", "Αποτύπωση οθόνης"),
        ("Input Control", "Έλεγχος εισόδου"),
        ("Audio Capture", "Λήψη ήχου"),
        ("File Connection", "Σύνδεση αρχείου"),
        ("Screen Connection", "Σύνδεση οθόνης"),
        ("Do you accept?", "Δέχεσαι;"),
        ("Open System Setting", "Άνοιγμα ρυθμίσεων συστήματος"),
        ("How to get Android input permission?", "Πώς να αποκτήσω άδεια εισαγωγής Android;"),
        ("android_input_permission_tip1", "Για να μπορεί μία απομακρυσμένη συσκευή να ελέγχει τη συσκευή σας Android, πρέπει να επιτρέψετε στο RustDesk να χρησιμοποιεί την υπηρεσία \"Προσβασιμότητα\"."),
        ("android_input_permission_tip2", "Παρακαλώ μεταβείτε στην επόμενη σελίδα ρυθμίσεων συστήματος, βρείτε και πληκτρολογήστε [Εγκατεστημένες υπηρεσίες], ενεργοποιήστε την υπηρεσία [Είσοδος RustDesk]."),
        ("android_new_connection_tip", "θέλω να ελέγξω τη συσκευή σου."),
        ("android_service_will_start_tip", "Η ενεργοποίηση της κοινής χρήσης οθόνης θα ξεκινήσει αυτόματα την υπηρεσία, ώστε άλλες συσκευές να μπορούν να ελέγχουν αυτήν τη συσκευή Android."),
        ("android_stop_service_tip", "Η απενεργοποίηση της υπηρεσίας θα αποσυνδέσει αυτόματα όλες τις εγκατεστημένες συνδέσεις."),
        ("android_version_audio_tip", "Η έκδοση Android που διαθέτετε δεν υποστηρίζει εγγραφή ήχου, ενημερώστε το σε Android 10 ή νεότερη έκδοση, εάν είναι δυνατόν."),
        ("android_start_service_tip", "Πατήστε [Ενεργοποίηση υπηρεσίας] ή ενεργοποιήστε την άδεια [Πρόσβαση στην οθόνη] για να ξεκινήσετε την υπηρεσία κοινής χρήσης οθόνης."),
        ("Account", "Λογαριασμός"),
        ("Overwrite", "Αντικατάσταση"),
        ("This file exists, skip or overwrite this file?", "Αυτό το αρχείο υπάρχει, παράβλεψη ή αντικατάσταση αυτού του αρχείου;"),
        ("Quit", "Έξοδος"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Βοήθεια"),
        ("Failed", "Απέτυχε"),
        ("Succeeded", "Επιτυχής"),
        ("Someone turns on privacy mode, exit", "Κάποιος ενεργοποιεί τη λειτουργία απορρήτου, έξοδος"),
        ("Unsupported", "Δεν υποστηρίζεται"),
        ("Peer denied", "Ο απομακρυσμένος σταθμός απέρριψε τη σύνδεση"),
        ("Please install plugins", "Παρακαλώ εγκαταστήστε πρόσθετα"),
        ("Peer exit", "Ο απομακρυσμένος σταθμός έχει αποσυνδεθεί"),
        ("Failed to turn off", "Αποτυχία απενεργοποίησης"),
        ("Turned off", "Απενεργοποιημένο"),
        ("In privacy mode", "Σε λειτουργία απορρήτου"),
        ("Out privacy mode", "Εκτός λειτουργίας απορρήτου"),
        ("Language", "Γλώσσα"),
        ("Keep RustDesk background service", "Εκτέλεση του RustDesk στο παρασκήνιο"),
        ("Ignore Battery Optimizations", "Παράβλεψη βελτιστοποιήσεων μπαταρίας"),
        ("android_open_battery_optimizations_tip", "Θέλετε να ανοίξετε τις ρυθμίσεις βελτιστοποίησης μπαταρίας;"),
        ("Connection not allowed", "Η σύνδεση απορρίφθηκε"),
        ("Legacy mode", "Λειτουργία συμβατότητας"),
        ("Map mode", "Map mode"),
        ("Translate mode", "Λειτουργία μετάφρασης"),
        ("Use permanent password", "Χρήση μόνιμου κωδικού πρόσβασης"),
        ("Use both passwords", "Χρήση και των δύο κωδικών πρόσβασης"),
        ("Set permanent password", "Ορισμός μόνιμου κωδικού πρόσβασης"),
        ("Enable Remote Restart", "Ενεργοποίηση απομακρυσμένης επανεκκίνησης"),
        ("Allow remote restart", "Να επιτρέπεται η απομακρυσμένη επανεκκίνηση"),
        ("Restart Remote Device", "Επανεκκίνηση απομακρυσμένης συσκευής"),
        ("Are you sure you want to restart", "Είστε βέβαιοι ότι θέλετε να κάνετε επανεκκίνηση"),
        ("Restarting Remote Device", "Επανεκκίνηση απομακρυσμένης συσκευής"),
        ("remote_restarting_tip", "Η απομακρυσμένη συσκευή επανεκκινείται, κλείστε αυτό το μήνυμα και επανασυνδεθείτε χρησιμοποιώντας τον μόνιμο κωδικό πρόσβασης."),
        ("Copied", "Αντιγράφηκε"),
        ("Exit Fullscreen", "Έξοδος από πλήρη οθόνη"),
        ("Fullscreen", "Πλήρης οθόνη"),
        ("Mobile Actions", "Mobile Actions"),
        ("Select Monitor", "Επιλογή οθόνης"),
        ("Control Actions", "Ενέργειες ελέγχου"),
        ("Display Settings", "Ρυθμίσεις οθόνης"),
        ("Ratio", "Αναλογία"),
        ("Image Quality", "Ποιότητα εικόνας"),
        ("Scroll Style", "Στυλ κύλισης"),
        ("Show Menubar", "Εμφάνιση γραμμής μενού"),
        ("Hide Menubar", "Απόκρυψη γραμμής μενού"),
        ("Direct Connection", "Απευθείας σύνδεση"),
        ("Relay Connection", "Αναμεταδιδόμενη σύνδεση"),
        ("Secure Connection", "Ασφαλής σύνδεση"),
        ("Insecure Connection", "Μη ασφαλής σύνδεση"),
        ("Scale original", "Κλιμάκωση πρωτότυπου"),
        ("Scale adaptive", "Προσαρμοστική κλίμακα"),
        ("General", "Γενικά"),
        ("Security", "Ασφάλεια"),
        ("Theme", "Θέμα"),
        ("Dark Theme", "Σκούρο θέμα"),
        ("Dark", "Σκούρο"),
        ("Light", "Φωτεινό"),
        ("Follow System", "Από το σύστημα"),
        ("Enable hardware codec", "Ενεργοποίηση κωδικοποιητή υλικού"),
        ("Unlock Security Settings", "Ξεκλείδωμα ρυθμίσεων ασφαλείας"),
        ("Enable Audio", "Ενεργοποίηση ήχου"),
        ("Unlock Network Settings", "Ξεκλείδωμα ρυθμίσεων δικτύου"),
        ("Server", "Διακομιστής"),
        ("Direct IP Access", "Άμεση πρόσβαση IP"),
        ("Proxy", "Διαμεσολαβητής"),
        ("Apply", "Εφαρμογή"),
        ("Disconnect all devices?", "Αποσύνδεση όλων των συσκευών;"),
        ("Clear", "Καθαρισμός"),
        ("Audio Input Device", "Συσκευή εισόδου ήχου"),
        ("Deny remote access", "Απόρριψη απομακρυσμένης πρόσβασης"),
        ("Use IP Whitelisting", "Χρήση λίστας επιτρεπόμενων IP"),
        ("Network", "Δίκτυο"),
        ("Enable RDP", "Ενεργοποίηση RDP"),
        ("Pin menubar", "Καρφίτσωμα γραμμής μενού"),
        ("Unpin menubar", "Ξεκαρφίτσωμα γραμμής μενού"),
        ("Recording", "Εγγραφή"),
        ("Directory", "Φάκελος εγγραφών"),
        ("Automatically record incoming sessions", "Αυτόματη εγγραφή εισερχόμενων συνεδριών"),
        ("Change", "Αλλαγή"),
        ("Start session recording", "Έναρξη εγγραφής συνεδρίας"),
        ("Stop session recording", "Διακοπή εγγραφής συνεδρίας"),
        ("Enable Recording Session", "Ενεργοποίηση εγγραφής συνεδρίας"),
        ("Allow recording session", "Να επιτρέπεται η εγγραφή"),
        ("Enable LAN Discovery", "Ενεργοποίηση εντοπισμού LAN"),
        ("Deny LAN Discovery", "Απαγόρευση εντοπισμού LAN"),
        ("Write a message", "Γράψτε ένα μήνυμα"),
        ("Prompt", "Προτροπή"),
        ("Please wait for confirmation of UAC...", "Παρακαλώ περιμένετε για επιβεβαίωση του UAC..."),
        ("elevated_foreground_window_tip", "Το τρέχον παράθυρο της απομακρυσμένης επιφάνειας εργασίας απαιτεί υψηλότερα δικαιώματα για να λειτουργήσει, επομένως δεν μπορεί να χρησιμοποιήσει προσωρινά το ποντίκι και το πληκτρολόγιο. Μπορείτε να ζητήσετε από τον απομακρυσμένο χρήστη να ελαχιστοποιήσει το τρέχον παράθυρο ή να κάνετε κλικ στο κουμπί ανύψωσης στο παράθυρο διαχείρισης σύνδεσης. Για να αποφύγετε αυτό το πρόβλημα, συνιστάται η εγκατάσταση του λογισμικού στην απομακρυσμένη συσκευή."),
        ("Disconnected", "Αποσυνδέθηκε"),
        ("Other", "Άλλα"),
        ("Confirm before closing multiple tabs", "Επιβεβαίωση πριν κλείσετε πολλές καρτέλες"),
        ("Keyboard Settings", "Ρυθμίσεις πληκτρολογίου"),
        ("Full Access", "Πλήρης πρόσβαση"),
        ("Screen Share", "Κοινή χρήση οθόνης"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Το Wayland απαιτεί Ubuntu 21.04 ή νεότερη έκδοση."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Το Wayland απαιτεί υψηλότερη έκδοση του linux distro. Δοκιμάστε την επιφάνεια εργασίας X11 ή αλλάξτε το λειτουργικό σας σύστημα."),
        ("JumpLink", "Προβολή"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Επιλέξτε την οθόνη που θέλετε να μοιραστείτε (Λειτουργία στην πλευρά του απομακρυσμένου σταθμού)."),
        ("Show RustDesk", "Εμφάνιση RustDesk"),
        ("This PC", "Αυτός ο υπολογιστής"),
        ("or", "ή"),
        ("Continue with", "Συνέχεια με"),
        ("Elevate", "Ανύψωση"),
        ("Zoom cursor", "Μεγέθυνση στον κέρσορα"),
        ("Accept sessions via password", "Αποδοχή συνεδριών μέσω κωδικού πρόσβασης"),
        ("Accept sessions via click", "Αποδοχή συνεδριών μέσω κλικ"),
        ("Accept sessions via both", "Αποδοχή συνεδριών και από τα δύο"),
        ("Please wait for the remote side to accept your session request...", "Παρακαλώ περιμένετε μέχρι η απομακρυσμένη πλευρά να αποδεχτεί το αίτημα συνεδρίας σας..."),
        ("One-time Password", "Κωδικός μίας χρήσης"),
        ("Use one-time password", "Χρήση κωδικού πρόσβασης μίας χρήσης"),
        ("One-time password length", "Μήκος κωδικού πρόσβασης μίας χρήσης"),
        ("Request access to your device", "Αίτημα πρόσβασης στη συσκευή σας"),
        ("Hide connection management window", "Απόκρυψη παραθύρου διαχείρισης σύνδεσης"),
        ("hide_cm_tip", "Να επιτρέπεται η απόκρυψη, μόνο εάν αποδέχεστε συνδέσεις μέσω κωδικού πρόσβασης και χρησιμοποιείτε μόνιμο κωδικό πρόσβασης"),
        ("wayland_experiment_tip", "Η υποστήριξη Wayland βρίσκεται σε πειραματικό στάδιο, χρησιμοποιήστε το X11 εάν χρειάζεστε πρόσβαση χωρίς επίβλεψη."),
        ("Right click to select tabs", "Κάντε δεξί κλικ για να επιλέξετε καρτέλες"),
        ("Skipped", "Παράλειψη"),
        ("Add to Address Book", "Προσθήκη στο Βιβλίο Διευθύνσεων"),
        ("Group", "Ομάδα"),
        ("Search", "Αναζήτηση"),
        ("Closed manually by web console", "Κλειστό χειροκίνητα από την κονσόλα web"),
        ("Local keyboard type", "Τύπος τοπικού πληκτρολογίου"),
        ("Select local keyboard type", "Επιλογή τύπου τοπικού πληκτρολογίου"),
        ("software_render_tip", "Εάν έχετε κάρτα γραφικών Nvidia και το παράθυρο σύνδεσης κλείνει αμέσως μετά τη σύνδεση, η εγκατάσταση του προγράμματος οδήγησης nouveau και η επιλογή χρήσης της επιτάχυνσης γραφικών μέσω λογισμικού μπορεί να βοηθήσει. Απαιτείται επανεκκίνηση."),
        ("Always use software rendering", "Επιτάχυνση γραφικών μέσω λογισμικού"),
        ("config_input", "Για να ελέγξετε την απομακρυσμένη επιφάνεια εργασίας με πληκτρολόγιο, πρέπει να εκχωρήσετε δικαιώματα στο RustDesk"),
        ("config_microphone", ""),
        ("request_elevation_tip", "αίτημα ανύψωσης δικαιωμάτων χρήστη"),
        ("Wait", "Περιμένετε"),
        ("Elevation Error", "Σφάλμα ανύψωσης δικαιωμάτων χρήστη"),
        ("Ask the remote user for authentication", "Ζητήστε από τον απομακρυσμένο χρήστη έλεγχο ταυτότητας"),
        ("Choose this if the remote account is administrator", "Επιλέξτε αυτό εάν ο απομακρυσμένος λογαριασμός είναι διαχειριστής"),
        ("Transmit the username and password of administrator", "Μεταβίβαση του ονόματος χρήστη και του κωδικού πρόσβασης του διαχειριστή"),
        ("still_click_uac_tip", "Εξακολουθεί να απαιτεί από τον απομακρυσμένο χρήστη να κάνει κλικ στο OK στο παράθυρο UAC όπου εκτελείται το RustDesk."),
        ("Request Elevation", "Αίτημα ανύψωσης δικαιωμάτων χρήστη"),
        ("wait_accept_uac_tip", "Περιμένετε να αποδεχτεί ο απομακρυσμένος χρήστης το παράθυρο διαλόγου UAC."),
        ("Elevate successfully", "Επιτυχής ανύψωση δικαιωμάτων χρήστη"),
        ("uppercase", "κεφαλαία γράμματα"),
        ("lowercase", "πεζά γράμματα"),
        ("digit", "Αριθμός"),
        ("special character", "ειδικός χαρακτήρας"),
        ("length>=8", "μήκος>=8"),
        ("Weak", "Αδύναμο"),
        ("Medium", "Μέτριο"),
        ("Strong", "Δυνατό"),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Closed as expected", ""),
        ("Guest to Host", ""),
        ("Dual way", ""),
        ("Audio Transmission Mode", ""),
    ].iter().cloned().collect();
}
