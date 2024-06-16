import $ from "jquery";
import Ajv from "ajv";

$(function () {
	$("#photo-field").on("change", function () {
		var file = this.files[0];
		console.info("File selected:", file.name);
		var maxSize = 2 * 1024 * 1024; // 2 MB
		var errorField = $("#photo-field-error");
		var uploadButton = $("#upload-button");
		if (file) {
			if (file.name.length > 64) {
				console.warn("File name exceeds 64 characters.");
				errorField.text("File name exceeds 64 characters.");
				$(this).val(""); // Clear the input
				uploadButton.prop("disabled", true);
			} else if (file.size > maxSize) {
				console.warn("File size exceeds 2 MB.");
				errorField.text("File size exceeds 2 MB.");
				$(this).val(""); // Clear the input
				uploadButton.prop("disabled", true);
			} else {
				console.info("File is valid.");
				errorField.text("");
				uploadButton.prop("disabled", false);
			}
		}
	});
	$("#upload-button").on("click", function () {
		var file = $("#photo-field")[0].files[0];
		var formData = new FormData();
		formData.append("photo", file);
		$.ajax({
			url: "/api/photos",
			type: "POST",
			data: formData,
			processData: false,
			contentType: false,
			success: function (data) {
				console.log("Upload successful:", data);
				if (data.path) {
					$("#profile-photo").attr("src", data.path);
					$("#photo-url").val(data.path);
				}
			},
			error: function (err) {
				console.error("Upload failed:", err);
			},
		});
	});
	$("#submit-button").on("click", function (event) {
		event.preventDefault();
		var payload = {
			first_name: $("#first_name").val(),
			last_name: $("#last_name").val(),
			email: $("#email").val(),
			bio: $("#bio").val(),
			intro: $("#intro").val(),
			photo_url: $("#photo-url").val(),
		};
		console.log("Creating author:", payload);

		var schema = JSON.parse($("#schema").val());
		const ajv = new Ajv({ allErrors: true });
		const validate = ajv.compile(schema);
		const valid = validate(payload);
		$("#errors").empty();
		if (!valid) {
			validate.errors.forEach((error) => {
				const errorMessage = document.createElement("p");
				errorMessage.classList.add("text-sm", "p-1");
				errorMessage.textContent = `${error.instancePath} ${error.message}`;
				$("#errors").append(errorMessage).show();
			});
		} else {
			$.ajax({
				url: "/admin/author",
				type: "POST",
				contentType: "application/json",
				data: JSON.stringify(payload),
				success: function (data) {
					console.log("Author created:", data);
					$("#errors").hide();
				},
				error: function (err) {
					console.error("Author creation failed:", err);
					const errorMessage = document.createElement("p");
					errorMessage.classList.add("text-sm", "p-1");
					errorMessage.textContent = `${err}`;
					$("#errors").append(errorMessage).show();
				},
			});
		}
	});
});
